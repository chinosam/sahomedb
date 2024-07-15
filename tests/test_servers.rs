mod common;

use common::{get_headers, run_server, stop_server};
use reqwest::Client;

const HOST: &str = "http://127.0.0.1";

// JSON body to create a new key-value store.
const CREATE_VALUE: &str = r#"{
    "key": "key-10",
    "value": {"embedding": [0.0, 0.0], "data": {}}
}"#;

const QUERY_INDEX: &str = r#"{
    "embedding": [0.0, 0.0],
    "count": 5
}"#;

#[tokio::test]
async fn test_get_root() {
    let port = String::from("31400");

    let runtime = run_server(port.clone()).await;

    let client = Client::new();
    let url = format!("{}:{}", HOST, port);
    let response = client.get(&url).send().await.unwrap();

    assert_eq!(response.status(), 200);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_post_values() {
    let port = String::from("31401");
    let runtime = run_server(port.clone()).await;

    // Create a key-value pair.
    let headers = get_headers();
    let url = format!("{}:{}/values", HOST, port);

    // Make a post request to create key-value store.
    let client = Client::new();
    let response = client
        .post(&url)
        .headers(headers)
        .body(CREATE_VALUE)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_get_values() {
    let port = String::from("31402");
    let runtime = run_server(port.clone()).await;

    let url = format!("{}:{}/values/key-0", HOST, port);

    let headers = get_headers();
    let client = Client::new();
    let response = client.get(url).headers(headers).send().await.unwrap();

    assert_eq!(response.status(), 200);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_delete_values() {
    let port = String::from("31403");
    let runtime = run_server(port.clone()).await;

    let url = format!("{}:{}/values/key-5", HOST, port);

    let headers = get_headers();
    let client = Client::new();
    let response = client.delete(&url).headers(headers).send().await.unwrap();

    assert_eq!(response.status(), 204);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_post_index() {
    let port = String::from("31404");

    let runtime = run_server(port.clone()).await;
    let url = format!("{}:{}/index", HOST, port);
    let headers = get_headers();

    let client = Client::new();
    let res = client.post(&url).headers(headers).send().await.unwrap();

    assert_eq!(res.status(), 200);
    stop_server(runtime).await;
}

#[tokio::test]
async fn test_post_index_query() {
    let port = String::from("31405");
    let runtime = run_server(port.clone()).await;

    // The body embedding is required and the dimension
    // must match the dimension specified in the config.
    let headers = get_headers();
    let url = format!("{}:{}/index/query", HOST, port);

    let client = Client::new();
    let res = client
        .post(&url)
        .headers(headers)
        .body(QUERY_INDEX)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    stop_server(runtime).await;
}
