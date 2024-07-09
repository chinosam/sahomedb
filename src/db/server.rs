use http::Response;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type RequestHeaders = HashMap<String, String>;

struct Request {
    pub method: String,
    pub route: String,
    pub headers: RequestHeaders,
    pub body: String,
}

// Arc and Mutex to share the key-value store
// across threads while ensuring exclusive access.
type KeyValue = Arc<Mutex<HashMap<String, String>>>;

pub struct Server {
    addr: SocketAddr,
    kv: KeyValue,
}

impl Server {
    pub async fn new(host: &str, port: &str) -> Server {
        let addr = format!("{}:{}", host, port).parse().unwrap();
        let kv = Arc::new(Mutex::new(HashMap::new()));
        Server { addr, kv }
    }

    pub async fn serve(&self) {
        // Bind a listner to the socket address
        let listner = TcpListener::bind(self.addr).await.unwrap();

        loop {
            let (stream, _) = listner.accept().await.unwrap();
            let handler = self.handle_connection(stream).await;
            tokio::spawn(async move { handler });
        }
    }
    async fn handle_connection(&self, mut stream: TcpStream) {
        loop {
            let req = self.read(&mut stream).await;

            if req.is_none() {
                break;
            }

            let req: &Request = req.as_ref().unwrap();
            let method = req.method.clone();
            let route = req.route.clone();

            // Handle the command for the response
            let response = match route.as_str() {
                "/" => self.handle_root(&method),
                "/version" => self.handle_version(&method),
                _ => self.get_not_found_res(),
            };

            // Write the data back to the client.
            self.write(&mut stream, response).await;
        }
    }

    fn handle_root(&self, method: &str) -> Response<String> {
        match method {
            "get" => self.get_root(),
            _ => self.get_not_allowed_res(),
        }
    }

    fn handle_version(&self, method: &str) -> Response<String> {
        match method {
            "get" => self.get_version(),
            _ => self.get_not_allowed_res(),
        }
    }

    // Route functions.
    // These are the functions that handle the route functionality
    // and is used by the handle_connection method.
    // Use format: _<method>_<route> for naming.

    fn get_root(&self) -> Response<String> {
        // Create a HashMap to store the status.
        let mut map = HashMap::new();
        map.insert("status", "ok");

        Response::builder()
            .status(200)
            .body(serde_json::to_string(&map).unwrap())
            .unwrap()
    }

    fn get_version(&self) -> Response<String> {
        let ver = env!("CARGO_PKG_VERSION");

        let mut map = HashMap::new();
        map.insert("version", ver);

        Response::builder()
            .status(200)
            .body(serde_json::to_string(&map).unwrap())
            .unwrap()
    }

    async fn read(&self, stream: &mut TcpStream) -> Option<Request> {
        let mut _headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut _headers);

        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();

        if n == 0 {
            return None;
        }

        let _ = req.parse(&buf).unwrap();

        let headers: RequestHeaders = HashMap::from_iter(req.headers.iter().map(|header| {
            let key = header.name.to_lowercase();
            let val = String::from_utf8_lossy(header.value).to_string();
            (key, val)
        }));

        let _content_len = headers
            .get("content-length")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()
            .unwrap_or(0);

        let body = if _content_len > 0 {
            let _buf = String::from_utf8_lossy(&buf);
            let _parts = _buf.split_once("\r\n\r\n").unwrap();
            _parts.1.replace("\0", "").clone()
        } else {
            String::new()
        };

        let data = Some(Request {
            method: req.method.unwrap().to_lowercase(),
            route: req.path.unwrap().to_string(),
            headers,
            body,
        });

        data
    }

    async fn write(&self, stream: &mut TcpStream, response: Response<String>) {
        let (parts, body) = response.into_parts();

        let status = parts.status.as_str();
        let reason = parts.status.canonical_reason().unwrap();

        let tag = format!("HTTP/1.1 {} {}", status, reason);
        let header = format!("content-length: {}", body.len());

        let data = format!("{}\r\n{}\r\n\r\n{}", tag, header, body);

        stream.write_all(data.as_bytes()).await.unwrap();
    }

    fn create_blank_res(&self, code: u16) -> Response<String> {
        let code = http::StatusCode::from_u16(code).unwrap();
        Response::builder()
            .status(code)
            .body("{}".to_string())
            .unwrap()
    }

    fn get_not_allowed_res(&self) -> Response<String> {
        self.create_blank_res(405)
    }

    fn get_not_found_res(&self) -> Response<String> {
        self.create_blank_res(404)
    }
}
