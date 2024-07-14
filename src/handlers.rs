use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use nom::AsBytes;

use crate::req::{HttpMethod, Request};
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


pub fn handle_file(req: &Request, param_map: &HashMap<String, String>) -> Response {
    let file_name = param_map.get("file_name");
    if file_name.is_some() {
        let mut path = PathBuf::from(env::var("download_dir").unwrap_or_default());
        path.push(file_name.unwrap());

        match req.method {
            HttpMethod::GET => {
                return handle_file_download(path)
            }
            HttpMethod::POST => {
                return handle_file_upload(req, &path)
            }
            _ => {
                Response::new(400, "Bad Request", "".as_bytes().to_vec())
            }
        };
    }
    Response::new(404, "Not Found", "".as_bytes().to_vec())
}


pub fn handle_file_download(path: PathBuf) -> Response {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();

            let mut resp = Response::new(200, "OK", contents);
            resp.content_type("application/octet-stream");
            return resp;
        }
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            Response::new(500, "Internal Server Error", "".as_bytes().to_vec())
        }
    }
}

pub fn handle_file_upload(req: &Request, path: &PathBuf) -> Response {
    if path.exists() {
        fs::remove_file(path).unwrap_or_default();
    }

    let mut file = File::create(path).unwrap();
    let content = req.body.as_bytes();
    file.write_all(content).unwrap();
    file.flush();

    Response::new(201, "Created", "".as_bytes().to_vec())
}
