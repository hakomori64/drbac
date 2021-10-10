use common::io;


mod state;
mod handlers;
mod request;
mod constants;

use common::connection::Connection;
use state::State;
use request::handle_request;


pub fn start_client() {
    let mut state = State::new(
        None,
        None,
    );
    let mut connection: Connection = Connection::new();
    loop {
        io::write(">> ");
        let command: String = io::read_line();

        state = match command.as_str() {
            "exit" | "quit" => {
                println!("shutting down...");
                break;
            },
            _ => {
                match handle_request(&mut connection, state.clone(), command.as_str()) {
                    Ok(state) => state,
                    Err(error) => {
                        println!("{}", error);
                        return;
                    }
                }
            }
        }
    }

}