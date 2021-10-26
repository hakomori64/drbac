use std::net::TcpStream;

// clientはどうにかしてentityがあるIPアドレスを見つけ出す

use common::connection::Connection;
use common::messages::HorizontalMessage;
use common::enums::ServerType;
use common::handlers::server::crypto_channel::crypto_channel;

mod state;
mod request;
mod handlers;

use state::State;


pub fn handle_connection(stream: TcpStream) {
    let mut connection: Connection = Connection::new();
    connection.set_stream(stream).expect("setting stream failed");
    let mut state = State::new();
    state = match crypto_channel(&mut connection, ServerType::Entity) {
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
        match connection.read_message::<HorizontalMessage>() {
            Ok(message) => {
                let result = request::handle_request(&mut connection, state, message);
                if let Err(error) = result {
                    if connection.write_message(&HorizontalMessage::Error {
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
                if connection.write_message(&HorizontalMessage::Error {
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