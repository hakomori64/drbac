use common::connection::Connection;
use common::messages::Message;
use anyhow::{Result, anyhow};

use super::handlers::crypto_channel::crypto_channel;
use super::handlers::identificate::identificate;
use super::handlers::whoami::whoami;
use super::handlers::roles::{
    delegate_role
};
use super::state::State;


pub fn handle_request(connection: &mut Connection, state: State, message: Message) -> Result<State> {

    match message {
        Message::CryptoChannelReq1 {..} => {
            match crypto_channel(connection, state, message) {
                Ok(state) => {
                    println!("暗号化通信の設定に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("暗号化通信の設定に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        Message::IdentificateReq1 {..} => {
            match identificate(connection, state, message) {
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
        Message::WhoamiReq1 {..} => {
            match whoami(connection, state, message) {
                Ok(state) => {
                    println!("身分確認に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("身分確認に成功しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        Message::DelegateRoleReq1 {..} => {
            match delegate_role(connection, state, message) {
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
        },
        /*
        "SEARCH_ROLE_REQ1" => {
            Some(state)
        }
        */
        _ => {
            return Err(anyhow!("認識できないリクエストです"));
        }
    }   
}