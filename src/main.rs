use sahomedb::db::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1", "3441").await;
    server.serve().await;
}
