mod common;

use common::{run_server, stop_server};
use reqwest::{get, Client};

const HOST: &str = "http://127.0.0.1";

// JSON body to create a new key-value store.
const CREATE_VALUE: &str = r#"{
    "key": "key-10",
    "value": {"embedding": [0.0, 0.0], "data": {}}
}"#;

const SEARCH: &str = r#"{
    "embedding": [0.0, 0.0],
    "count": 5
}"#;

#[tokio::test]
async fn test_get_root() {
    let port = String::from("31400");

    let url = format!("{}:{}", HOST, port);
    let runtime = run_server(port).await;

    let res = get(url).await.unwrap();
    assert_eq!(res.status(), 200);

    stop_server(runtime).await;
}

#[tokio::test]
async fn test_post_values() {
    let port = String::from("31401");
    let url = format!("{}:{}/values", HOST, port);
    let runtime = run_server(port).await;

    // Make a post request to create key-value store.
    let client = Client::new();
    let res = client.post(&url).body(CREATE_VALUE).send().await.unwrap();

    // Assert the response code.
    assert_eq!(res.status(), 201);

    stop_server(runtime).await;
}

#[tokio::test]
async fn test_get_values() {
    let port = String::from("31402");

    let url = format!("{}:{}/values/key-0", HOST, port);

    let runtime = run_server(port).await;

    // Call GET to get the value of the key.
    let res = get(url).await.unwrap();
    // Assert the response code.
    assert_eq!(res.status(), 200);

    stop_server(runtime).await;
}

#[tokio::test]
async fn test_delete_values() {
    let port = String::from("31403");

    let url = format!("{}:{}/values/key-5", HOST, port);

    let runtime = run_server(port).await;

    let client = Client::new();
    let res = client.delete(&url).send().await.unwrap();

    assert_eq!(res.status(), 204);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_post_build() {
    let port = String::from("31404");

    // Build the index.
    let url = format!("{}:{}/build", HOST, port);
    let runtime = run_server(port).await;
    let client = Client::new();

    let res = client.post(&url).send().await.unwrap();
    assert_eq!(res.status(), 200);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_post_search() {
    let port = String::from("31405");

    // Make a post request to search for nearest neighbors.
    let url = format!("{}:{}/search", HOST, port);
    let runtime = run_server(port).await;

    // The body embedding is required and the dimension
    // must match the dimension specified in the config.
    let client = Client::new();
    let res = client.post(&url).body(SEARCH).send().await.unwrap();

    assert_eq!(res.status(), 200);
    stop_server(runtime).await;
}
