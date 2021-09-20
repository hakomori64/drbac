use common::encoding::{value_to_struct};
use common::connection::Connection;
use common::messages::Message;
use common::messages::handlers::{
    crypto_channel,
    identificate
};
use anyhow::{Result, anyhow};

use super::handlers::crypto_channel::crypto_channel;
use super::handlers::identificate::identificate;
use super::state::State;


pub fn handle_request(connection: &mut Connection, state: State, message: Message) -> Result<State> {

    let mut new_state: Option<State> = None;
    println!("{}", message.header);
    new_state = match message.header.as_str() {
        "CRYPTO_CHANNEL_REQ1" => {
            let data = value_to_struct::<crypto_channel::CryptoChannelReq1>(message.data)?;
            match crypto_channel(connection, state, &data) {
                Ok(state) => Some(state),
                Err(_) => return Err(anyhow!("暗号化通信の設定に失敗しました"))
            }
        }
        "IDENTIFICATE_REQ1" => {
            let data = value_to_struct::<identificate::IdentificateReq1>(message.data)?;
            match identificate(connection, state, &data) {
                Ok(state) => Some(state),
                Err(_) => return Err(anyhow!("身分証明に失敗しました"))
            }
        }
        "WHOAMI_REQ1" => {
            Some(state)
        }
        "DELEGATE_ROLE_REQ1" => {
            Some(state)
        }
        "SEARCH_ROLE_REQ1" => {
            Some(state)
        }
        _ => {
            connection.close("認識できないリクエストです").expect("closing connection failed");
            return Err(anyhow!("認識できないリクエストが渡された"));
        }
    };
    
    match new_state {
        Some(data) => Ok(data),
        None => Err(anyhow!("stateの更新がなされませんでした"))
    }
}