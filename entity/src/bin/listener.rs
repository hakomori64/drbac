use common::{socket_server::SocketServer, utils::set_current_dir_to_executable_directory};
use entity::listener::handle_connection;
use std::env;

fn main() {
    println!("starting client server...");
    set_current_dir_to_executable_directory().unwrap();

    let args = env::args();
    if args.len() != 3 {
        println!("USAGE: listener --enable_jail <true | false>");
    }
    let socket_server = SocketServer::new("localhost", 8081);
    socket_server.start(handle_connection);
}