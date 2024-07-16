use rand::random;
use reqwest::header::HeaderMap;
use sahomedb::db::routes::handle_request;
use sahomedb::db::server::{Config, Server, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

pub async fn run_server(port: String) -> Runtime {
    let runtime = Runtime::new().unwrap();

    runtime.spawn(async move {
        let host = "127.0.0.1";
        let port = port.as_str();
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

        let listener = TcpListener::bind(addr).await.unwrap();
        let (mut stream, _) = listener.accept().await.unwrap();

        let config = Config { dimension: 2, token: "token".to_string() };

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

pub async fn stop_server(runtime: Runtime) {
    runtime.shutdown_background();
}

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-sahomedb-token", "token".parse().unwrap());
    headers
}
