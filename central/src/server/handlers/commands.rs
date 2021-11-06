use anyhow::{anyhow, Result};
use common::messages::VerticalMessage;
use common::connection::Connection;
use super::super::state::State;
use std::net::TcpStream;
use common::handlers::client::crypto_channel::crypto_channel;


pub fn execute_command(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::ExecuteReq1 { box_name, command, args } = data {
        let mut entity_connection = Connection::new();

        if state.opponent_actor().is_none() {
            return Err(anyhow!("actorの認証が済んでいません"));
        }

        match TcpStream::connect(format!("{}:{}", box_name, 8081)) {
            Ok(stream) => {
                entity_connection.set_stream(stream).expect("setting stream failed");
            }
            Err(_) => {
                return Err(anyhow!("実行先へのコネクションの確立に失敗しました"));
            }
        };

        match crypto_channel(&mut entity_connection, state.clone()) {
            Ok(_) => {
                println!("実行先との暗号化に成功しました");
            }
            Err(err) => {
                println!("実行先との暗号化に失敗しました");
                println!("{}", err);
            }
        }

        entity_connection.write_message(&VerticalMessage::ExecuteProxyReq1 {
            actor: state.opponent_actor().unwrap(),
            command,
            args
        })?;

        let result = match entity_connection.read_message()? {
            VerticalMessage::ExecuteProxyRes1 { result } => result,
            _ => return Err(anyhow!("実行先でエラーが発生しました"))
        };

        connection.write_message(&VerticalMessage::ExecuteRes1 {
            result: result
        })?;

        Ok(state)
    } else {
        return Err(anyhow!("ExecuteReq1が渡されませんでした"));
    }
}