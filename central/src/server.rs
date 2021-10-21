use std::net::TcpStream;
use std::net::TcpListener;
use std::path::PathBuf;

use common::thread_pool::ThreadPool;
use common::connection::Connection;
use common::messages::Message as DRBACMessage;
use common::pki::{
    read_pem,
    create_pem,
    generate_key_pair,
};

mod request;
mod state;
mod handlers;

use state::State;


pub fn start_server(host: &str, port: i32) {
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    
    let pool = ThreadPool::new(5);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let secret_path: PathBuf = ["secret_key.pem"].iter().collect();
    let public_path: PathBuf = ["public_key.pem"].iter().collect();
    let (secret_key, public_key) = if secret_path.exists() && public_path.exists() {
        (read_pem(&secret_path).unwrap(), read_pem(&public_path).unwrap())
    } else if !secret_path.exists() && !public_path.exists() {
        let (secret_key, public_key) = generate_key_pair().unwrap();
        create_pem(&secret_path, String::from("secret key"), secret_key.clone()).unwrap();
        create_pem(&public_path, String::from("public key"), public_key.clone()).unwrap();
        (secret_key, public_key)
    } else {
        panic!("key error");
    };


    let mut connection: Connection = Connection::new();
    connection.set_stream(stream).expect("setting stream failed");
    let mut state = State::new(
        None,
        Some(secret_key),
        Some(public_key),
    );
    loop {
        println!("reading from stream...");
        match connection.read_message() {
            Ok(message) => {
                let result = request::handle_request(&mut connection, state, message);
                if let Err(error) = result {
                    if connection.write_message(&DRBACMessage::Error {
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
                if connection.write_message(&DRBACMessage::Error {
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