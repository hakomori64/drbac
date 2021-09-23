use common::connection::Connection;
use common::messages::Message;
use anyhow::{Result, anyhow};

use super::handlers::crypto_channel::crypto_channel;
use super::handlers::identificate::identificate;
use super::state::State;


pub fn handle_request(connection: &mut Connection, state: State, message: Message) -> Result<State> {

    match message {
        Message::CryptoChannelReq1 {..} => {
            match crypto_channel(connection, state, message) {
                Ok(state) => Ok(state),
                Err(_) => return Err(anyhow!("暗号化通信の設定に失敗しました"))
            }
        }
        Message::IdentificateReq1 {..} => {
            match identificate(connection, state, message) {
                Ok(state) => Ok(state),
                Err(_) => return Err(anyhow!("身分証明に失敗しました"))
            }
        }
        /*
        "WHOAMI_REQ1" => {
            Some(state)
        }
        "DELEGATE_ROLE_REQ1" => {
            Some(state)
        }
        "SEARCH_ROLE_REQ1" => {
            Some(state)
        }
        */
        _ => {
            return Err(anyhow!("認識できないリクエストです"));
        }
    }   
}