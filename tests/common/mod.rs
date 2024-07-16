use rand::random;
use reqwest::header::HeaderMap;
use sahomedb::db::routes::handle_request;
use sahomedb::db::server::{Config, Server, Value};
use std::collections::HashMap;
use std::fs::remove_dir_all;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

const DATA_DIR: &str = "tests/data";

pub async fn run_server(port: String) -> Runtime {
    let runtime = Runtime::new().unwrap();

    runtime.spawn(async move {
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        let listener = TcpListener::bind(addr).await.unwrap();
        let (mut stream, _) = listener.accept().await.unwrap();

        let config = {
            let dimension = 2;
            let token = "token".to_string();
            let path = format!("{}/{}", DATA_DIR, port);
            Config { dimension, token, path }
        };

        let server = Server::new(config);

        // Pre-populate the key-value store.
        for i in 0..9 {
            // Generate value with random embeddings.
            let value = Value {
                embedding: vec![random::<f32>(); 2],
                data: HashMap::new(),
            };

            // Set the key-value pair.
            let key = format!("key-{}", i);
            server.set(key, value).unwrap();
        }

        let ef = 10; // small EF for testing only.
        server.build(ef, ef).unwrap();

        handle_request(&server, &mut stream).await;
    });

    // Return runtime as a handle to stop the server.
    runtime
}

pub async fn stop_server(runtime: Runtime, port: String) {
    runtime.shutdown_background();

    remove_dir_all(format!("{}/{}", DATA_DIR, port)).unwrap();
}

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-sahomedb-token", "token".parse().unwrap());
    headers
}
