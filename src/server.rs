use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use anyhow::Context;

use crate::req::parse_request;
use crate::route::Router;

pub struct Server {
    addr: String,
    router: Arc<Router>,
}


impl Server {
    pub fn new(addr: &str, router: Router) -> Self {
        return Server {
            addr: addr.to_owned(),
            router: Arc::new(router),
        };
    }

    pub fn run(self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(_stream) => {
                    let router = Arc::clone(&self.router);
                    thread::spawn(move || {
                        match handle_client(_stream, router) {
                            Ok(_) => {}
                            Err(err) => {
                                println!("Failed to handle client: {}", err);
                            }
                        }
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}


fn handle_client(mut stream: TcpStream, router: Arc<Router>) -> anyhow::Result<()> {
    match parse_request(&mut stream) {
        Ok(req) => {
            let resp = router.handle_req(&req);
            let content = &resp.to_bytes();
            stream.write_all(content).context("Failed to write response")?;
            stream.flush()?
        }
        Err(err) => {
            println!("request parse error: {}", err);
        }
    }

    Ok(())
}
