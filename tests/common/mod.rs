use sahomedb::db::server::Server;
use tokio::runtime::Runtime;

pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "31415";

pub async fn run_server() -> Runtime {
    let runtime = Runtime::new().unwrap();

    runtime.spawn(async move {
        let server = Server::new(HOST, PORT).await;
        server.serve().await;
    });

    runtime
}

pub async fn stop_server(runtime: Runtime) {
    runtime.shutdown_background();
}
