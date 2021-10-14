use std::env;
use central::server;
use entity::client;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("main (central | entity)");
        return;
    }
    match args[1].as_str() {
        "central" => {
            println!("starting central server...");
            server::start_server("localhost", 8080);
        }
        "entity" => {
            println!("starting client...");
            client::start_client();
        }
        _ => {
            println!("main (central entity)");
        }
    }
    return;
}
