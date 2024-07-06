use std::{io::{Read, Write}, net::TcpListener};
use std::net::TcpStream;

use anyhow::Context;

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

    let path = first_line.splitn(3, " ")
        .nth(1)
        .context("No path in request")?;

    let resp = if path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    stream.write(resp.as_bytes())?;
    Ok(())
}
