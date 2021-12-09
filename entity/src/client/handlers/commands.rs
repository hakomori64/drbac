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

    let entity_id: String = io::read_until(
        "コマンドを実行するエンティティのIDを入力してください：",
        "正しいIDを入力してください",
        |_| true
    );

    loop {
        let operation: String = io::read_until(
            "実行コマンドを入力してください: ",
            "正しいホスト名を入力してください",
            |_| true
        );

        if operation.as_str() == "exit" {
            connection.write_message(&VerticalMessage::ExecuteReq2 {})?;
            match connection.read_message()? {
                VerticalMessage::ExecuteRes2 {} => {
                    println!("command execution finished");
                    return Ok(state);
                },
                _ => {
                    return Err(anyhow!("ExecuteRes2でないレスポンスを受け取りました"));
                }
            }
        }

        let role_id: String = io::read_until(
            "コマンドを実行するロールを入力してください：",
            "正しいIDを入力してください",
            |_| true
        );
    
        let commands: Vec<String> = operation.split_whitespace().map(|s| String::from(s)).collect();
    
        connection.write_message(&VerticalMessage::ExecuteReq1 {
            box_name: host_name.clone(),
            entity_id: entity_id.clone(),
            command: commands.clone(),
            role_id: role_id,
        })?;
    
        match connection.read_message()? {
            VerticalMessage::ExecuteRes1 { result } => {
                println!("### result ###");
                println!("{}", result);
            }
            _ => {
                return Err(anyhow!("ExecuteRes1でないレスポンスを受け取りました"));
            }
        }
    }
}