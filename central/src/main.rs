mod server;
mod stream;

fn main() {
    server::start_server("localhost", 8080);
}
