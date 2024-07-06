use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0; 1024];
                match _stream.read(&mut buf) {
                    Ok(bytes_read) if bytes_read > 0 => {
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
                            _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");
                        } else {
                            _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").expect("200 \n");
                        }
                    }
                    Ok(_) => {}
                    Err(e) => println!("Read error: {}", e),
                };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
