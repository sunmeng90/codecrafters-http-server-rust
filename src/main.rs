use std::io::{Read, Write};

use anyhow::Context;

use crate::handlers::{handle_echo, handle_user_agent};
use crate::route::Router;
use crate::server::Server;

pub(crate) mod req;
mod resp;
mod route;
mod handlers;
mod server;

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let mut router = Router::new();
    router.add_route("/echo/:echo", handle_echo);
    router.add_route("/user-agent", handle_user_agent);

    Server::new("127.0.0.1:4221", router).run();

    Ok(())
}
