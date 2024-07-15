use super::utils::response as res;
use super::utils::stream;
use crate::db::server::Server;
use tokio::net::TcpStream;

pub mod index;
pub mod root;
pub mod values;
pub mod version;

// In this module, we define the route handlers
// for the database server HTTP API.

pub async fn handle_connection(server: &mut Server, stream: &mut TcpStream) {
    loop {
        handle_request(server, stream).await;
    }
}

async fn handle_request(server: &mut Server, stream: &mut TcpStream) {
    let req = stream::read(stream).await;

    if req.is_none() {
        let response = res::get_error_response(400, "Invalid request.");
        stream::write(stream, response).await;
        return;
    }

    let request = req.as_ref().unwrap();
    let route = request.route.clone();

    // Check if the route is private.
    // Private routes require authentication.
    let private_routes = ["/index", "/values"];
    if private_routes.iter().any(|r| route.starts_with(r)) {
        // Get the token from the request headers.
        let token = request.headers.get("x-sahomedb-token");

        if token.is_none() || token.unwrap() != &server.config.token {
            let response = res::get_401_response();
            stream::write(stream, response).await;
            return;
        }
    }

    let response = match route.as_str() {
        "/" => root::handler(request),
        "/version" => version::handler(request),
        _ if route.starts_with("/index") => index::handler(server, request),
        _ if route.starts_with("/values") => values::handler(server, request),
        _ => res::get_404_response(),
    };

    // Write the data back to the client.
    stream::write(stream, response).await;
}
