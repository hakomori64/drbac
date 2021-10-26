use anyhow::{Result};
use common::messages::HorizontalMessage;
use common::connection::Connection;
use super::state::State;

use super::handlers::crypto_channel::crypto_channel;

pub fn handle_request(connection: &mut Connection, state: State, message: HorizontalMessage) -> Result<State> {
    match message {
        HorizontalMessage::CryptoChannelReq1 {..} => {
            match crypto_channel(connection, state, message) {
                Ok(state) => {
                    println!("暗号化の通信に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("暗号化通信の設定に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        _ => {
            Ok(state)
        }
    }
}