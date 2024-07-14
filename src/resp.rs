use std::collections::HashMap;
use std::ops::Add;

use nom::AsBytes;

pub struct Response {
    pub status_code: u16,
    pub reason_phrase: &'static str,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}


impl Response {
    pub fn new(status_code: u16, reason_phrase: &'static str, body: Vec<u8>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        Response {
            status_code,
            reason_phrase,
            headers,
            body: body.to_owned(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut resp = String::new();
        let first_line = format!("HTTP/1.1 {} {}", self.status_code, self.reason_phrase);
        resp = resp.add(&first_line.as_str());
        self.headers.iter().for_each(
            |(k, v)| {
                let header_line = format!("{}: {}", k, v);
                resp = resp.clone().add(&header_line.as_str());
            }
        );
        let mut resp = resp.as_bytes().to_vec();
        resp.extend(&self.body);
        resp.to_vec()
    }
}