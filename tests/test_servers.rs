mod common;

use common::*;
use serde_json::Value;

#[tokio::test]
async fn test_get_root() {
    let runtime = run_server().await;

    let url: String = format!("http://{}:{}/", HOST, PORT);
    let res = reqwest::get(url).await.unwrap();

    let code = res.status();
    let body = res.text().await.unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(code, 200);
    assert_eq!(json.get("status").unwrap(), "ok");

    stop_server(runtime).await;
}
