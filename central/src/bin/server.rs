use common::{utils::set_current_dir_to_executable_directory, socket_server::SocketServer};
use central::server::handle_connection;

fn main() {
    println!("starting central server...");
    set_current_dir_to_executable_directory().unwrap();
    let socket_server = SocketServer::new("0.0.0.0", 8080);
    socket_server.start(handle_connection);
}
