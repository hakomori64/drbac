use anyhow::{Result, anyhow};
use common::messages::VerticalMessage;
use common::connection::Connection;
use super::state::State;
use super::handlers::commands::{
    execute_command
};

pub fn handle_request(connection: &mut Connection, state: State, message: VerticalMessage) -> Result<State> {
    match message {
        VerticalMessage::ExecuteProxyReq1 {..} => {
            match execute_command(connection, state, message) {
                Ok(state) => {
                    println!("コマンドの実行に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("コマンドの実行に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        _ => {
            Err(anyhow!("予期しないリクエストです"))
        }
    }
}