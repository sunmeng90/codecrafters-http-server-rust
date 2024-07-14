use std::env;


use crate::handlers::{handle_echo, handle_file, handle_user_agent};
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

    let args: Vec<String> = env::args().skip(1).collect();

    let mut dir = String::new();
    if args.len() < 2 {
        eprintln!("insufficient arguments");
    } else {
        let arg_name = args.get(0).unwrap();
        if arg_name == "--directory" {
            dir = args.get(1).unwrap().to_string();
            env::set_var("download_dir", dir);
        }
    }

    let mut router = Router::new();
    router.add_route("/echo/:echo", handle_echo);
    router.add_route("/user-agent", handle_user_agent);
    router.add_route("/files/:file_name", handle_file);

    Server::new("127.0.0.1:4221", router).run();

    Ok(())
}
