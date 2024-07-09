use sahomedb::db::server::Server;
use tokio::runtime::Runtime;

pub async fn run_server() -> (Runtime, String) {
    let runtime = Runtime::new().unwrap();

    // Generate a random port: 314xx.
    // This is needed to run multiple tests in parallel and
    // prevent connection reset error when testing.
    let random_number = rand::random::<u16>() % 100 + 31400;
    let port = random_number.to_string();
    let _port = port.clone();

    runtime.spawn(async move {
        let server = Server::new("127.0.0.1", _port.as_str()).await;
        server.serve().await;
    });

    (runtime, port)
}

pub async fn stop_server(runtime: Runtime) {
    runtime.shutdown_background();
}
