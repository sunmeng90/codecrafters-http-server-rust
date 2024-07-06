use std::{io::{Read, Write}, net::TcpListener};
use std::net::TcpStream;

use anyhow::Context;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => handle_client(_stream)?,
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let first_line = std::str::from_utf8(&buf[..bytes_read])
        .context("Invalid utf-8")?
        .lines()
        .next()
        .context("No first line in request")?;

    println!("First line in req [{}]", first_line);

    let reg_pattern = r"/echo/([^/]+)";

    let path = first_line.splitn(3, " ")
        .nth(1)
        .context("Failed to get path")?;

    println!("Path [{}] in req", path);

    if path == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .context("Failed to write response")?;
        return Ok(());
    }

    let re = Regex::new(reg_pattern).context("Failed to compile echo pattern")?;


    match re.captures(path) {
        Some(captures) => {
            let echo_str = captures.get(1)
                .context("Failed to get a capture group")?
                .as_str()
                .to_string();

            println!("echo [{}] to client", echo_str);

            let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_str.len(), echo_str);
            stream.write(resp.as_bytes())
                .context("Failed to write response")?;
        }
        None => {
            println!("Request path not matched");
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                .context("Failed to write response")?;
        }
    }

    Ok(())
}
