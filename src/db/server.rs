use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

use super::routes::kvs;
use super::routes::root;
use super::routes::version;
use super::utils::response as res;
use super::utils::stream;

// Data type for the key-value store value metadata.
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

pub struct Server {
    addr: SocketAddr,
    kvs: KeyValue,
}

impl Server {
    pub async fn new(host: &str, port: &str) -> Server {
        let addr = format!("{}:{}", host, port).parse().unwrap();
        let kvs = Arc::new(Mutex::new(HashMap::new()));

        Server { addr, kvs }
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
        let mut kvs = self.kvs.lock().unwrap();
        kvs.insert(key, value.clone());

        Ok(value)
    }

    pub fn delete(&self, key: String) -> Result<Value, &str> {
        let mut kvs = self.kvs.lock().unwrap();
        kvs.remove(&key).ok_or("The key doesn't exist.")
    }
}
