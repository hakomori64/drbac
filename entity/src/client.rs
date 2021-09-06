use text_io::read;

mod state;
mod connection;

use state::State;
use connection::connect;

pub fn start_client() {
    let mut state: Option<State> = None;
    loop {
        let command: String = read!("{}\n");
    
        match command.as_str() {
            "connect" => {
                print!("host: string = ");
                let host: String = read!("{}\n");
                print!("port: int = ");
                let port: String = read!("{}\n");
                let port: i32 = port.parse::<i32>().unwrap();
                match connect(&mut state, host.as_str(), port) {
                    Ok(()) => {
                        println!("コネクションの確立に成功しました。");
                    }
                    Err(_) => {
                        println!("コネクションの確立に失敗しました")
                    }
                }
            }
            "encrypt channel" => {}
            "identificate" => {}
            "whoami" => {}
            "delegate role" => {}
            "search role" => {}
            "generate key" => {}
            "exit" | "quit" => {
                println!("shutting down");
                break;
            }
            _ => {
                println!("認識されていないコマンドです");
            }
        }
    }
}