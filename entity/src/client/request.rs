use anyhow::{Result};

use common::io;
use super::state::State;
use super::handlers::connection::connect;
use super::handlers::crypto_channel::crypto_channel;
use super::handlers::identificate::identificate;
use super::handlers::whoami::whoami;
use super::handlers::roles::{
    delegate_role
};
use super::constants;
use common::connection::Connection;

pub fn handle_request(connection: &mut Connection, state: State, command: &str) -> Result<State> {
    match command {
        "connect" => {
            io::write("host: string = ");
            let host: String = io::read_line();
            io::write("port: int = ");
            let port: i32 = io::read_line();
            match connect(connection, state.clone(), host.as_str(), port) {
                Ok(state) => {
                    println!("コネクションの確立に成功しました。");
                    Ok(state)
                }
                Err(err) => {
                    println!("コネクションの確立に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "encrypt channel" => {
            match crypto_channel(connection, state.clone()) {
                Ok(state) => {
                    println!("通信の暗号化に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("通信の暗号化に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "identificate" => {
            match identificate(connection, state.clone()) {
                Ok(state) => {
                    println!("身分証明に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("身分証明に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "whoami" => {
            match whoami(connection, state.clone()) {
                Ok(state) => {
                    println!("身分確認に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("身分確認に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "delegate role" => {
            match delegate_role(connection, state.clone()) {
                Ok(state) => {
                    println!("ロールの付与に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("ロールの付与に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "search role" => {
            Ok(state)
        }
        "generate key" => {
            Ok(state)
        }
        "help" | _ => {
            println!("{}", constants::HELP_TEXT);
            Ok(state)
        }
    }
}