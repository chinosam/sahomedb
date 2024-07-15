use dotenv::dotenv;
use sahomedb::db::server::{Config, Server};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("SAHOMEDB_PORT").unwrap_or(String::from("3141"));

    let dimension = env_get_dimension();

    let config = Config { dimension };

    println!("SahomeDB is running on port {}.", port);
    println!("SahomeDB accepts embeddings of {} dimension.", dimension);

    let host = "0.0.0.0";
    let mut server = Server::new(host, port.as_str(), config);
    server.serve().await;
}

fn env_get_dimension() -> usize {
    let not_set = "env variable 'SAHOMEDB_DIMENSION' required";
    let not_int = "variable 'SAHOMEDB_DIMENSION' must be an integer";
    env::var("SAHOMEDB_DIMENSION")
        .expect(not_set)
        .parse::<usize>()
        .expect(not_int)
}
