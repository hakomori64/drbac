use anyhow::{anyhow, Result};
use common::messages::VerticalMessage;
use common::connection::Connection;
use super::super::state::State;
use std::net::TcpStream;
use common::handlers::client::crypto_channel::crypto_channel;
use common::db::utils::establish_connection;
use common::db::models::delegation::{
    get_roles,
};
use common::db::models::actor::Actor;


pub fn execute_command(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::ExecuteReq1 { box_name, entity_id, command, args } = data {
        let mut entity_connection = Connection::new();

        if state.opponent_actor().is_none() {
            return Err(anyhow!("actorの認証が済んでいません"));
        }

        let db_conn = establish_connection()?;
        let actor = state.opponent_actor().unwrap();
        let roles = get_roles(&db_conn, &actor)?;

        // 関係のあるロールだけ抽出する
        let mut related_roles: Vec<Actor> = vec![];
        for role in roles {
            let role_parent_id = if let Actor::Role { entity_id, .. } = role.clone() {
                entity_id
            } else {
                return Err(anyhow!("get_rolesがロール以外のActorを返しました"));
            };

            if entity_id == role_parent_id {
                related_roles.push(role);
            }
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
                entity_connection.close()?;
            }
        }

        match entity_connection.write_message(&VerticalMessage::ExecuteProxyReq1 {
            actor: state.opponent_actor().unwrap(),
            entity_id: entity_id.clone(),
            command,
            args,
            roles: related_roles.clone(),
        }) {
            Err(err) => {
                println!("実行先へのコマンドの送信に失敗しました");
                println!("{}", err);
                entity_connection.close()?;
            },
            _ => ()
        };

        let result = match entity_connection.read_message() {
            Ok(message) => {
                match message {
                    VerticalMessage::ExecuteProxyRes1 { result } => result,
                    _ => {
                        println!("実行先からの結果の受信に失敗しました");
                        entity_connection.close()?;
                        return Err(anyhow!("実行先からの結果の受信に失敗しました"));
                    }
                }
            },
            Err(err) => {
                println!("実行先からの結果の受信に失敗しました");
                println!("{}", err);
                entity_connection.close()?;
                return Err(anyhow!("実行先からの結果の受信に失敗しました"));
            }
        };

        connection.write_message(&VerticalMessage::ExecuteRes1 {
            result: result
        })?;

        loop {
            let message = match connection.read_message::<VerticalMessage>() {
                Ok(message) => message,
                Err(err) => {
                    println!("実行結果の読み込みに失敗しました");
                    println!("{}", err);
                    entity_connection.close()?;
                    return Err(anyhow!("実行結果の読み込みに失敗しました"));
                }
            };

            match message {
                VerticalMessage::ExecuteReq1 { command, args, .. } => {
                    match entity_connection.write_message(&VerticalMessage::ExecuteProxyReq1 {
                        actor: state.opponent_actor().unwrap(),
                        entity_id: entity_id.clone(),
                        command,
                        args,
                        roles: related_roles.clone(),
                    }) {
                        Err(err) => {
                            println!("実行先へのコマンドの送信に失敗しました");
                            println!("{}", err);
                            entity_connection.close()?;
                        },
                        _ => ()
                    };
            
                    let result = match entity_connection.read_message() {
                        Ok(message) => {
                            match message {
                                VerticalMessage::ExecuteProxyRes1 { result } => result,
                                _ => {
                                    println!("実行先からの結果の受信に失敗しました");
                                    entity_connection.close()?;
                                    return Err(anyhow!("実行先からの結果の受信に失敗しました"));
                                }
                            }
                        },
                        Err(err) => {
                            println!("実行先からの結果の受信に失敗しました");
                            println!("{}", err);
                            entity_connection.close()?;
                            return Err(anyhow!("実行先からの結果の受信に失敗しました"));
                        }
                    };
            
                    connection.write_message(&VerticalMessage::ExecuteRes1 {
                        result: result
                    })?;
                },
                VerticalMessage::ExecuteReq2 {} => {
                    match entity_connection.write_message(&VerticalMessage::ExecuteProxyReq2 {}) {
                        Err(err) => {
                            println!("実行先でエラーが発生しました");
                            println!("{}", err);
                            entity_connection.close()?;
                            return Err(anyhow!("実行先でエラーが発生しました"));
                        },
                        _ => {}
                    };
                    match entity_connection.read_message::<VerticalMessage>() {
                        Err(err) => {
                            println!("実行先でエラーが発生しました");
                            println!("{}", err);
                            entity_connection.close()?;
                            return Err(anyhow!("実行先でエラーが発生しました"));
                        },
                        _ => {}
                    }
                    entity_connection.close()?;
                    connection.write_message(&VerticalMessage::ExecuteRes2 {})?;
                    return Ok(state);
                },
                _ => {
                    return Err(anyhow!("予期しないリクエストが来ました"));
                }
            }
        }
    } else {
        return Err(anyhow!("ExecuteReq1が渡されませんでした"));
    }
}