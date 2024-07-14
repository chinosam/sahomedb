use super::request::{Request, RequestHeaders};
use super::response::Response;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn read(stream: &mut TcpStream) -> Option<Request> {
    let mut _headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut _headers);

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await.unwrap();

    if n == 0 {
        return None;
    }

    let _ = req.parse(&buf).unwrap();

    let headers: RequestHeaders =
        HashMap::from_iter(req.headers.iter().map(|header| {
            let key = header.name.to_lowercase();
            let val = String::from_utf8_lossy(header.value).to_string();
            (key, val)
        }));

    let content_len = headers
        .get("content-length")
        .unwrap_or(&"0".to_string())
        .parse::<usize>()
        .unwrap_or(0);

    let body = if content_len > 0 {
        let buf = String::from_utf8_lossy(&buf);
        let parts = buf.split_once("\r\n\r\n").unwrap();
        parts.1.replace('\0', "").clone()
    } else {
        "{}".to_string()
    };

    let body = match serde_json::from_str(&body) {
        Ok(body) => body,
        Err(_) => None,
    };

    body.as_ref()?;

    Some(Request {
        method: req.method.unwrap().to_lowercase(),
        route: req.path.unwrap().to_string(),
        headers,
        body: body.unwrap(),
    })
}

pub async fn write(stream: &mut TcpStream, response: Response<String>) {
    let (parts, body) = response.into_parts();

    let status = parts.status.as_str();
    let reason = parts.status.canonical_reason().unwrap();

    let tag = format!("HTTP/1.1 {} {}", status, reason);
    let header = format!("content-length: {}", body.len());

    let data = format!("{}\r\n{}\r\n\r\n{}", tag, header, body);

    stream.write_all(data.as_bytes()).await.unwrap();
}
