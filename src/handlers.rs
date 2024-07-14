use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use nom::AsBytes;

use crate::req::Request;
use crate::resp::Response;

pub fn handle_base(req: &Request, param_map: &HashMap<String, String>) -> Response {
    Response::new(200, "OK", "".as_bytes().to_vec())
}
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


pub fn handle_file_download(req: &Request, param_map: &HashMap<String, String>) -> Response {
    if let Some(filename) = param_map.get("file_name") {
        let mut path = PathBuf::from(env::var("download_dir").unwrap_or_default());
        path.push(filename);
        let mut file = File::open(path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();

        let mut resp = Response::new(200, "OK", contents);
        resp.content_type("application/octet-stream");
        resp
    } else {
        Response::new(400, "Bad Request", "".as_bytes().to_vec())
    }
}


