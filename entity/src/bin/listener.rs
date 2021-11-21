use common::socket_server::SocketServer;
use entity::listener::handle_connection;
use std::env;

fn main() {
    println!("starting client server...");
    let args = env::args();
    if args.len() != 3 {
        println!("USAGE: listener --enable_jail <true | false>");
    }
    let socket_server = SocketServer::new("localhost", 8081);
    socket_server.start(handle_connection);
}