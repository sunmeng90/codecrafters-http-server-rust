use std::{io::{Read, Write}, net::TcpListener};
use std::net::TcpStream;

use anyhow::Context;

use crate::handlers::{handle_echo, handle_user_agent};
use crate::req::parse_request;
use crate::route::Router;

pub(crate) mod req;
mod resp;
mod route;
mod handlers;

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let mut router = Router::new();

    router.add_route("/echo", handle_echo);
    router.add_route("/user-agent", handle_user_agent);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => match handle_client(_stream, &router) {
                Ok(_) => {}
                Err(err) => {
                    println!("failed to handle client: {}", err);
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, router: &Router) -> anyhow::Result<()> {
    match parse_request(&mut stream) {
        Ok(req) => {
            let resp = router.handle_req(&req);
            stream.write_all(&resp.to_bytes()).context("Failed to write response")?;
            stream.flush()?
        }
        Err(err) => {
            println!("request parse error: {}", err);
        }
    }

    Ok(())
}
