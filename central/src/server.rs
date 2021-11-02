use std::net::TcpStream;
use std::path::PathBuf;

use common::connection::Connection;
use common::messages::VerticalMessage as DRBACVerticalMessage;
use common::pki::{
    read_pem,
    read_json,
    Certificate,
};
use common::pki::BoxType;

mod request;
mod state;
mod handlers;

use state::State;
use common::handlers::server::crypto_channel::crypto_channel;


pub fn handle_connection(stream: TcpStream) {
    let secret_path: PathBuf = ["secret_key.pem"].iter().collect();
    let public_path: PathBuf = ["public_key.pem"].iter().collect();
    let cert_path: PathBuf = ["cert.json"].iter().collect();
    let (secret_key, public_key) = if secret_path.exists() && public_path.exists() {
        (read_pem(&secret_path).unwrap(), read_pem(&public_path).unwrap())
    } else {
        panic!("key error");
    };

    let certificate: Certificate = if cert_path.exists() {
        read_json(&cert_path).unwrap()
    } else {
        panic!("certificate not found error");
    };

    let mut connection: Connection = Connection::new();
    connection.set_stream(stream).expect("setting stream failed");
    let mut state = State::new(
        None,
        Some(secret_key),
        Some(public_key),
        Some(certificate)
    );
    // encrypt channel here
    state = match crypto_channel(&mut connection, BoxType::Central) {
        Ok(_) => {
            println!("通信の暗号化に成功しました");
            state
        },
        Err(err) => {
            println!("通信の暗号化に失敗しました");
            println!("{}", err);
            return;
        }
    };

    loop {
        println!("reading from stream...");
        match connection.read_message() {
            Ok(message) => {
                let result = request::handle_request(&mut connection, state, message);
                if let Err(error) = result {
                    if connection.write_message(&DRBACVerticalMessage::Error {
                        reason: error.to_string()
                    }).is_err() {
                        break;
                    }
                    if connection.close().is_err() {
                        break;
                    }
                    break;
                }
                state = result.unwrap();
            },
            Err(error) => {
                if connection.write_message(&DRBACVerticalMessage::Error {
                    reason: error.to_string()
                }).is_err() {
                    println!("writing message failed");
                    break;
                }
                if connection.close().is_err() {
                    println!("closing connection failed");
                    break;
                }
                break;
            }
        }
    }
    println!("リクエストのハンドリングを終了します");
}