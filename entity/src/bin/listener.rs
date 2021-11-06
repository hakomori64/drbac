use common::socket_server::SocketServer;
use entity::listener::handle_connection;

fn main() {
    println!("starting client server...");
    let socket_server = SocketServer::new("localhost", 8081);
    socket_server.start(handle_connection);
}