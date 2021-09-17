use common::io;


mod state;
mod handlers;

use state::State;
use handlers::connection::connect;
use handlers::crypto_channel::crypto_channel;
use common::connection::Connection;


pub fn start_client() {
    let mut state = State::new();
    let mut connection: Connection = Connection::new();
    loop {
        let mut new_state: Option<State> = None;
        io::write(">> ");
        let command: String = io::read_line();
    
        match command.as_str() {
            "connect" => {
                io::write("host: string = ");
                let host: String = io::read_line();
                io::write("port: int = ");
                let port: i32 = io::read_line();
                match connect(&mut connection, state, host.as_str(), port) {
                    Ok(data) => {
                        new_state = Some(data);
                        println!("コネクションの確立に成功しました。");
                    }
                    Err(_) => {
                        println!("コネクションの確立に失敗しました")
                    }
                }
            }
            "encrypt channel" => {
                match crypto_channel(&mut connection, state) {
                    Ok(data) => {
                        new_state = Some(data);
                        println!("通信の暗号化に成功しました");
                    }
                    Err(_) => {
                        println!("通信の暗号化に失敗しました");
                    }
                }
            }
            "identificate" => {}
            "whoami" => {}
            "delegate role" => {}
            "search role" => {}
            "generate key" => {}
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
            }
        }

        if new_state.is_some() {
            state = new_state.unwrap();
        }
    }
}