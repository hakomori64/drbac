use anyhow::{Result, anyhow};

use common::io;
use common::connection::Connection;
use super::super::state::State;
use common::messages::VerticalMessage;

pub fn execute_command(connection: &mut Connection, state: State) -> Result<State> {
    let host_name: String = io::read_until(
        "コマンドを実行するホスト名を入力してください: ",
        "正しいホスト名を入力してください",
        |_| true
    );

    let operation: String = io::read_until(
        "実行コマンドを入力してください: ",
        "正しいホスト名を入力してください",
        |_| true
    );

    let mut commands: Vec<String> = operation.split_whitespace().map(|s| String::from(s)).collect();

    connection.write_message(&VerticalMessage::ExecuteReq1 {
        box_name: host_name,
        command: commands[0].clone(),
        args: commands.drain(1..).collect()
    })?;

    match connection.read_message()? {
        VerticalMessage::ExecuteRes1 { result } => {
            println!("### result ###");
            println!("{}", result);

            Ok(state)
        }
        _ => {
            return Err(anyhow!("ExecuteRes1でないレスポンスを受け取りました"));
        }
    }
}