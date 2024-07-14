use std::collections::HashMap;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub fn parse_request(stream: &mut TcpStream) -> Result<Request, &'static str> {
    let mut reader = BufReader::new(stream);
    let mut headers = HashMap::new();
    let mut buffer = String::new();

    // read the request line
    reader.read_line(&mut buffer).map_err(|_| "Failed to read request line")?;

    let req_line = buffer.trim();
    let mut parts = req_line.split_whitespace();

    let method_str = parts.next().ok_or("Failed to parse method")?;
    let uri = parts.next().ok_or("Failed to parse URI")?.to_string();
    let http_version_str = parts.next().ok_or("Failed to parse HTTP version")?;

    let method = HttpMethod::from_str(method_str).map_err(|_| "Invalid HTTP method")?;
    let http_version = HttpVersion::from_str(http_version_str).map_err(|_| "Invalid HTTP version")?;

    buffer.clear();

    // read headers
    loop {
        reader.read_line(&mut buffer).map_err(|_| "Failed to read header")?;
        let line = buffer.trim();
        if line.is_empty() {
            break;
        }
        let mut header_parts = line.splitn(2, ": ");
        let header_name = header_parts.next().ok_or("Failed to parse header name")?.to_string();
        let header_val = header_parts.next().ok_or("Failed to parse header value")?.to_string();
        headers.insert(header_name, header_val);
        buffer.clear();
    }

    let mut len = if let Some(len) = headers.get("Content-Length") {
        len.parse::<usize>().unwrap()
    } else {
        0
    };

    // read the body
    let mut buf = [0; 1024];
    let mut body = Vec::new();

    while len > 0 {
        let read_bytes_count = reader.read(&mut buf).map_err(|_| "Failed to read the body")?;
        if read_bytes_count > 0 {
            body.extend_from_slice(&buf[..read_bytes_count]);
            len -= read_bytes_count;
        }
    }

    Ok(Request {
        method,
        uri,
        http_version,
        headers,
        body,
    })
}


#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<HttpMethod, ()> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "PATCH" => Ok(HttpMethod::PATCH),
            "TRACE" => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
}

impl FromStr for HttpVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<HttpVersion, ()> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2.0" => Ok(HttpVersion::Http20),
            _ => Err(()),
        }
    }
}
