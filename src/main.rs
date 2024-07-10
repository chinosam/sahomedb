use sahomedb::db::server::Server;

#[tokio::main]
async fn main() {
    let host = "127.0.0.1";
    let port = "3141";
    let dimension = 3;

    let mut server = Server::new(host, port, dimension).await;
    server.serve().await;
}
