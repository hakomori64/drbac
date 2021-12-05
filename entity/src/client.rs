use common::io;


mod state;
mod handlers;
mod request;
mod constants;

use common::connection::Connection;
use common::state::StateTrait;
use state::State;
use request::handle_request;
use handlers::connection::connect;
use common::io::{
    read_json_from_file,
};
use common::pki::{
    BoxType,
    Certificate,
    read_pem,
};
use common::handlers::client::crypto_channel::crypto_channel;
use std::path::PathBuf;


pub fn start_client() {
    let secret_path: PathBuf = ["secret_key.pem"].iter().collect();
    let public_path: PathBuf = ["public_key.pem"].iter().collect();
    let cert_path: PathBuf = ["cert.json"].iter().collect();
    let (secret_key, public_key) = if secret_path.exists() && public_path.exists() {
        (read_pem(&secret_path).unwrap(), read_pem(&public_path).unwrap())
    } else {
        panic!("key error");
    };

    let certificate: Certificate = if cert_path.exists() {
        read_json_from_file(&cert_path).unwrap()
    } else {
        panic!("certificate not found error");
    };

    //let (secret_key, public_key) = 
    let mut state = State::new(
        None,
        Some(secret_key),
        Some(public_key),
        Some(certificate),
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
                state = match connect(&mut connection, state.clone(), host.as_str(), port) {
                    Ok(state) => {
                        println!("コネクションの確立に成功しました。");
                        state
                    }
                    Err(error) => {
                        println!("コネクションの確立に失敗しました");
                        println!("{}", error);
                        return;
                    }
                };

                // encrypt channel here
                match crypto_channel(&mut connection, state.clone()) {
                    Ok(opponent_type) => {
                        println!("通信の暗号化に成功しました");
                        State::new(
                            state.actor(),
                            state.box_secret_key(),
                            state.box_public_key(),
                            state.box_certificate(),
                            Some(opponent_type),
                        )
                    }
                    Err(err) => {
                        println!("通信の暗号化に失敗しました");
                        println!("{}", err);
                        return;
                    }
                }
            },
            "exit" | "quit" => {
                println!("shutting down...");
                break;
            },
            _ => {
                let opponent_type = state.opponent_type().unwrap();
                match opponent_type {
                    BoxType::Central => {
                        match handle_request(&mut connection, state.clone(), command.as_str()) {
                            Ok(state) => state,
                            Err(error) => {
                                println!("{}", error);
                                return;
                            }
                        }
                    }
                    BoxType::Client => {
                        // entity command handling comes here
                        state
                    }
                }
            }
        }
    }
}