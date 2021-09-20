use common::io;


mod state;
mod handlers;

use state::State;
use handlers::connection::connect;
use handlers::crypto_channel::crypto_channel;
use handlers::identificate::identificate;
use common::connection::Connection;


pub fn start_client() {
    let mut state = State::new();
    let mut connection: Connection = Connection::new();
    loop {
        let mut new_state: Option<State> = None;
        io::write(">> ");
        let command: String = io::read_line();
    
        new_state = match command.as_str() {
            "connect" => {
                io::write("host: string = ");
                let host: String = io::read_line();
                io::write("port: int = ");
                let port: i32 = io::read_line();
                match connect(&mut connection, state, host.as_str(), port) {
                    Ok(state) => {
                        println!("コネクションの確立に成功しました。");
                        Some(state)
                    }
                    Err(err) => {
                        println!("コネクションの確立に失敗しました");
                        println!("{}", err);
                        None
                    }
                }
            }
            "encrypt channel" => {
                match crypto_channel(&mut connection, state) {
                    Ok(state) => {
                        println!("通信の暗号化に成功しました");
                        Some(state)
                    }
                    Err(err) => {
                        println!("通信の暗号化に失敗しました");
                        println!("{}", err);
                        None
                    }
                }
            }
            "identificate" => {
                match identificate(&mut connection, state) {
                    Ok(state) => {
                        println!("身分証明に成功しました");
                        Some(state)
                    }
                    Err(err) => {
                        println!("身分証明に失敗しました");
                        println!("{}", err);
                        None
                    }
                }
            }
            "whoami" => {
                Some(state)
            }
            "delegate role" => {
                Some(state)
            }
            "search role" => {
                Some(state)
            }
            "generate key" => {
                Some(state)
            }
            "exit" | "quit" => {
                println!("shutting down");
                break;
            }
            "help" | _ => {
                let data = r#"
commands
- connect
- encrypt channel
- identificate
- whoami
- delegate role
- generate key
- exit
- quit
- help
                "#;
                println!("{}", data);
                None
            }
        };

        if new_state.is_some() {
            state = new_state.unwrap();
        }
    }
}