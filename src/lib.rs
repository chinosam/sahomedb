pub mod db;

use crate::db::routes::handle_request;
use crate::db::server::{Config, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[macro_use]
extern crate serde_derive;

pub async fn serve(host: &str, port: &str, config: Config) {
    let server = Arc::new(Server::new(config));

    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let server = server.clone();
        tokio::spawn(async move {
            handle_request(&server, &mut stream).await;
        });
    }
}
