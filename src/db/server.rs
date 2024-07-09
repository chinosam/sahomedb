use http::Response;
use serde_json::Value as RequestBody;
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
    pub body: RequestBody,
}

// This type will be used to serialize the response body.
type ResponseBody = HashMap<&'static str, &'static str>;

// This is the data structure that will be stored in
// the key-value store as the value.
#[derive(Serialize, Deserialize, Debug)]
struct Value {
    embedding: Vec<f32>,
    data: HashMap<String, String>,
}

// Arc and Mutex to share the key-value store
// across threads while ensuring exclusive access.
type KeyValue = Arc<Mutex<HashMap<String, Value>>>;

pub struct Server {
    addr: SocketAddr,
    kvs: KeyValue,
}

impl Server {
    pub async fn new(host: &str, port: &str) -> Server {
        let addr = format!("{}:{}", host, port).parse().unwrap();
        let kvs = Arc::new(Mutex::new(HashMap::new()));
        Server { addr, kvs }
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

            // Handle disconnection or invalid request.
            // Return invalid request response.
            if req.is_none() {
                let mut res_body = HashMap::new();
                res_body.insert("error", "Invalid request.");
                let res: Response<String> = self.create_res(400, Some(res_body));
                break;
            }

            let req: &Request = req.as_ref().unwrap();
            let route = req.route.clone();

            // Handle the command for the response
            let response = match route.as_str() {
                "/" => self.handle_root(req),
                "/version" => self.handle_version(req),
                _ if route.starts_with("/kvs") => self.handle_kvs(req),
                _ => self.get_not_found_res(),
            };

            // Write the data back to the client.
            self.write(&mut stream, response).await;
        }
    }

    fn handle_root(&self, request: &Request) -> Response<String> {
        match request.method.as_str() {
            "get" => self.get_root(),
            _ => self.get_not_allowed_res(),
        }
    }

    fn handle_version(&self, request: &Request) -> Response<String> {
        match request.method.as_str() {
            "get" => self.get_version(),
            _ => self.get_not_allowed_res(),
        }
    }

    fn handle_kvs(&self, request: &Request) -> Response<String> {
        match request.method.as_str() {
            "post" => self.post_kvs(request.body.clone()),
            _ => self.get_not_allowed_res(),
        }
    }

    // Route functions.
    // These are the functions that handle the route functionality
    // and is used by the handle_connection method.
    // Use format: _<method>_<route> for naming.

    fn get_root(&self) -> Response<String> {
        let mut map = HashMap::new();
        map.insert("status", "ok");

        self.create_res(200, Some(map))
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

    fn post_kvs(&self, request_body: RequestBody) -> Response<String> {
        // If request body is missing key or value.
        if request_body.get("key").is_none() || request_body.get("value").is_none() {
            let mut _map = HashMap::new();
            _map.insert("error", "Both key and value are required.");
            return self.create_res(400, Some(_map));
        }

        // Get the key from request body.
        // Validate that key is string.
        let key: String = match request_body["key"].as_str() {
            Some(key) => key.to_string(),
            None => {
                let mut _map = HashMap::new();
                _map.insert("error", "The key must be a string.");
                return self.create_res(400, Some(_map));
            }
        };

        // Get the value from request body.
        // Validate that value is a Value struct.
        let value: Value = match serde_json::from_value(request_body["value"].clone()) {
            Ok(value) => value,
            Err(_) => {
                let mut _map = HashMap::new();
                let msg = "The value provided is invalid.";
                _map.insert("error", msg);
                return self.create_res(400, Some(_map));
            }
        };

        // Insert the key-value pair into the key-value store.
        let mut kvs = self.kvs.lock().unwrap();
        kvs.insert(key, value);

        // Serialize value as string for the response.
        let body = {
            let _val: Value = serde_json::from_value(request_body["value"].clone()).unwrap();
            serde_json::to_string(&_val).unwrap()
        };

        Response::builder().status(201).body(body).unwrap()
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

        let content_len = headers
            .get("content-length")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()
            .unwrap_or(0);

        let body = if content_len > 0 {
            let _buf = String::from_utf8_lossy(&buf);
            let _parts = _buf.split_once("\r\n\r\n").unwrap();
            _parts.1.replace("\0", "").clone()
        } else {
            "{}".to_string()
        };

        let body: Option<RequestBody> = match serde_json::from_str(&body) {
            Ok(body) => body,
            Err(_) => None,
        };

        if body.is_none() {
            return None;
        }

        let data = Some(Request {
            method: req.method.unwrap().to_lowercase(),
            route: req.path.unwrap().to_string(),
            headers,
            body: body.unwrap(),
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

    fn create_res(&self, code: u16, body: Option<ResponseBody>) -> Response<String> {
        let code = http::StatusCode::from_u16(code).unwrap();
        let body = if !body.is_none() {
            serde_json::to_string(&body.unwrap()).unwrap()
        } else {
            "{}".to_string()
        };

        Response::builder().status(code).body(body).unwrap()
    }

    fn get_not_allowed_res(&self) -> Response<String> {
        self.create_res(405, None)
    }

    fn get_not_found_res(&self) -> Response<String> {
        self.create_res(404, None)
    }
}
