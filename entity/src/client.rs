use common::io;


mod state;
mod handlers;
mod request;
mod constants;

use common::connection::Connection;
use state::State;
use request::handle_request;
use handlers::connection::connect;


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
            "connect" => {
                io::write("host: string = ");
                let host: String = io::read_line();
                io::write("port: int = ");
                let port: i32 = io::read_line();
                match connect(&mut connection, state.clone(), host.as_str(), port) {
                    Ok(state) => {
                        println!("コネクションの確立に成功しました。");
                        state
                    }
                    Err(error) => {
                        println!("コネクションの確立に失敗しました");
                        println!("{}", error);
                        return;
                    }
                }
            },
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