use common::socket_server::SocketServer;
use central::server::handle_connection;

fn main() {
    println!("starting central server...");
    let socket_server = SocketServer::new("localhost", 8080);
    socket_server.start(handle_connection);
}