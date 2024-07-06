use std::{io::{Read, Write}, net::TcpListener};
use std::net::TcpStream;
fn main() -> Result<(), std::io::Error> {
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

fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let path = String::from_utf8_lossy(&buf[..bytes_read])
        .lines()
        .next()
        .expect("should have first line in request")
        .splitn(3, " ")
        .skip(1)
        .next()
        .to_owned()
        .unwrap()
        .to_owned();
    if path == "/" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").expect("200 \n");
    }
    Ok(())
}
