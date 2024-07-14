use std::collections::HashMap;

use nom::AsBytes;

use crate::req::Request;
use crate::resp::Response;

pub fn handle_404(req: &Request) -> Response {
    Response::new(404, "Not Found", "404 Not Found".as_bytes().to_vec())
}

pub fn handle_echo(req: &Request, param_map: &HashMap<String, String>) -> Response {
    if let Some(echo) = param_map.get("echo") {
        Response::new(200, "OK", echo.as_bytes().to_vec())
    } else {
        Response::new(400, "Bad Request", "".as_bytes().to_vec())
    }
}

pub fn handle_user_agent(req: &Request, param_map: &HashMap<String, String>) -> Response {
    if let Some(agent) = req.headers.get("User-Agent") {
        Response::new(200, "OK", agent.as_bytes().to_vec())
    } else {
        Response::new(400, "Bad Request", "".as_bytes().to_vec())
    }
}

