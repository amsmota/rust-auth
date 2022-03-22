#![allow(dead_code)]

mod server;
mod http;
mod auth_handler;
mod zkp_server;
mod zkp_client;
mod zkp_structs;

use server::Server;
use auth_handler::AuthHandler;
use std::env;

fn main() {
    let default_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(AuthHandler::new());
}
