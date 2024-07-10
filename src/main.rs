use sahomedb::db::server::{Config, Server};

#[tokio::main]
async fn main() {
    let host = "127.0.0.1";
    let port = "3141";

    let config = Config { dimension: 2 };

    let mut server = Server::new(host, port, config).await;
    server.serve().await;
}
