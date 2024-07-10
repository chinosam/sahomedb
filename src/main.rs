use sahomedb::db::server::Server;

#[tokio::main]
async fn main() {
    let host = "127.0.0.1";
    let port = "3141";

    let mut server = Server::new(host, port).await;
    server.serve().await;
}
