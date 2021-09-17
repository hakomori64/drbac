use common::encoding::{value_to_struct};
use common::connection::Connection;
use common::messages::Message;
use common::messages::handlers::{
    crypto_channel
};
use anyhow::{Result, anyhow};

use super::handlers::crypto_channel::crypto_channel;
use super::state::State;


pub fn handle_request(connection: &mut Connection, state: State, message: Message) -> Result<State> {

    let mut new_state: Option<State> = None;
    println!("{}", message.req_type);
    match message.req_type.as_str() {
        "CRYPTO_CHANNEL_REQ1" => {
            let data = value_to_struct::<crypto_channel::CryptoChannelReq1>(message.data)?;
            new_state = match crypto_channel(connection, state, &data) {
                Ok(data) => Some(data),
                Err(_) => return Err(anyhow!("暗号化チャネルの構築中にエラーが発生しました"))
            }
        }
        "AUTH_IDENTIFICATE_REQ1" => {}
        "WHOAMI_REQ1" => {}
        "DELEGATE_ROLE_REQ1" => {}
        "SEARCH_ROLE_REQ1" => {}
        _ => {
            connection.close("認識できないリクエストです").expect("closing connection failed");
            return Err(anyhow!("認識できないリクエストが渡された"));
        }
    }
    
    match new_state {
        Some(data) => Ok(data),
        None => Err(anyhow!("stateの更新がなされませんでした"))
    }
}