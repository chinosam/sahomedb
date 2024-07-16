use dotenv::dotenv;
use sahomedb::db::server::Config;
use sahomedb::serve;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("SAHOMEDB_PORT").unwrap_or(String::from("3141"));

    let dimension = env_get_dimension();

    let config = {
        let token = get_env("SAHOMEDB_TOKEN");
        Config { dimension, token }
    };

    println!("SahomeDB is running on port {}.", port);
    println!("SahomeDB accepts embeddings of {} dimension.", dimension);

    let host = "0.0.0.0";
    serve(host, &port, config).await;
}

fn env_get_dimension() -> usize {
    get_env("SAHOMEDB_DIMENSION")
        .parse::<usize>()
        .expect("variable 'SAHOMEDB_DIMENSION' must be an integer")
}

fn get_env(key: &str) -> String {
    let not_set = format!("env variable '{}' required", key);
    env::var(key).expect(&not_set)
}
