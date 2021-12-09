use common::{socket_server::SocketServer, utils::set_current_dir_to_executable_directory};
use entity::listener::handle_connection;

fn main() {
    println!("starting client server...");
    set_current_dir_to_executable_directory().unwrap();

    let socket_server = SocketServer::new("0.0.0.0", S8081);
    socket_server.start(handle_connection);
}