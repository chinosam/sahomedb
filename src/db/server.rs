use instant_distance::HnswMap as HNSW;
use instant_distance::{Builder, Search};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

use super::routes::build;
use super::routes::kvs;
use super::routes::root;
use super::routes::search;
use super::routes::version;

use super::utils::response as res;
use super::utils::stream;

// Data type for the key-value store value's metadata.
pub type Data = HashMap<String, String>;

// This is the data structure that will be stored in
// the key-value store as the value.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub embedding: Vec<f32>,
    pub data: Data,
}

// Arc and Mutex to share the key-value store
// across threads while ensuring exclusive access.
type KeyValue = Arc<Mutex<HashMap<String, Value>>>;

type Index = Option<HNSW<Value, String>>;

// db
pub struct Config {
    pub dimension: usize,
}

pub struct Server {
    addr: SocketAddr,
    kvs: KeyValue,
    index: Index,
    config: Config,
}

impl Server {
    pub fn new(host: &str, port: &str, config: Config) -> Server {
        let addr = format!("{}:{}", host, port).parse().unwrap();
        let kvs = Arc::new(Mutex::new(HashMap::new()));

        let index: Index = None;

        Server { addr, kvs, index, config }
    }

    pub async fn serve(&mut self) {
        // Bind a listner to the socket address
        let listner = TcpListener::bind(self.addr).await.unwrap();

        loop {
            let (stream, _) = listner.accept().await.unwrap();
            let handler = self.handle_connection(stream).await;
            tokio::spawn(async move { handler });
        }
    }

    async fn handle_connection(&mut self, mut stream: TcpStream) {
        loop {
            let req = stream::read(&mut stream).await;

            // Handle disconnection or invalid request.
            // Return invalid request response.
            if req.is_none() {
                let res = res::get_error_response(400, "Invalid request.");
                stream::write(&mut stream, res).await;
                break;
            }

            let req = req.as_ref().unwrap();
            let route = req.route.clone();

            // Handle the command for the response
            let response = match route.as_str() {
                "/" => root::handler(req),
                "/version" => version::handler(req),
                "/build" => build::handler(self, req),
                "/search" => search::handler(self, req),
                _ if route.starts_with("/kvs") => kvs::handler(self, req),
                _ => res::get_404_response(),
            };

            // Write the data back to the client.
            stream::write(&mut stream, response).await;
        }
    }

    // Native functionality handler.
    // These are the functions that handle the native
    // functionality of the database.
    // Example: get, set, delete, etc.

    pub fn get(&self, key: String) -> Result<Value, &str> {
        let kvs = self.kvs.lock().unwrap();
        kvs.get(&key).cloned().ok_or("The value is not found.")
    }

    pub fn set(&mut self, key: String, value: Value) -> Result<Value, &str> {
        if value.embedding.len() != self.config.dimension {
            return Err("The embedding dimension is invalid.");
        }

        let mut kvs = self.kvs.lock().unwrap();
        kvs.insert(key, value.clone());
        Ok(value)
    }

    pub fn delete(&self, key: String) -> Result<Value, &str> {
        let mut kvs = self.kvs.lock().unwrap();
        kvs.remove(&key).ok_or("The key doesn't exist.")
    }

    // Index functionality handler.
    pub fn build(
        &mut self,
        ef_search: usize,
        ef_construction: usize,
    ) -> Result<&str, &str> {
        // Clear the current index
        self.index = None;

        // Get the key-value store.
        let kvs = self.kvs.lock().unwrap();

        // Separate key-value to keys and values.
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (key, value) in kvs.iter() {
            keys.push(key.clone());
            values.push(value.clone());
        }

        // Build and set the index.
        let index = Builder::default()
            .ef_search(ef_search)
            .ef_construction(ef_construction)
            .build(values, keys);

        self.index = Some(index);
        Ok("The index is built successfully.")
    }

    pub fn search(
        &self,
        embedding: Vec<f32>,
        count: usize,
    ) -> Result<Vec<Data>, &str> {
        // Validate the dimension of the embedding.
        if embedding.len() != self.config.dimension {
            return Err("The embedding dimension is invalid.");
        }

        let index = self.index.as_ref().ok_or("The index is not built.")?;

        // Create a decoy value with the provided embedding.
        let point = Value { embedding, data: HashMap::new() };

        let mut search = Search::default();
        let results = index.search(&point, &mut search);

        let mut data: Vec<Data> = Vec::new();
        for result in results {
            let value = result.point;
            data.push(value.data.clone());
        }

        data.truncate(count);

        Ok(data)
    }
}

// This is the implementation of the Point trait.
// This is needed by the library to calculate the distance
// between two vectors.
impl instant_distance::Point for Value {
    fn distance(&self, other: &Self) -> f32 {
        let mut sum = 0.0;

        // Implement Euclidean distance formula.
        for i in 0..self.embedding.len() {
            sum += (self.embedding[i] - other.embedding[i]).powi(2);
        }

        sum.sqrt()
    }
}
