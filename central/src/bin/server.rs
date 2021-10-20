use central::server;

fn main() {
    println!("starting central server...");
    server::start_server("localhost", 8080);
}