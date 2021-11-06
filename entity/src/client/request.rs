use anyhow::{Result};

use super::state::State;
use super::handlers::identificate::identificate;
use super::handlers::whoami::whoami;
use super::handlers::roles::{
    delegate_role,
    search_roles,
};
use super::handlers::commands::{
    execute_command
};
use super::handlers::registration::{
    register_entity,
    register_role,
    register_user,
};
use super::constants;
use common::connection::Connection;

pub fn handle_request(connection: &mut Connection, state: State, command: &str) -> Result<State> {
    match command {
        "identificate" => {
            match identificate(connection, state.clone()) {
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
        "whoami" => {
            match whoami(connection, state.clone()) {
                Ok(state) => {
                    println!("身分確認に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("身分確認に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "delegate role" => {
            match delegate_role(connection, state.clone()) {
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
        }
        "search role" => {
            match search_roles(connection, state.clone()) {
                Ok(state) => {
                    println!("ロールの検索に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("ロールの検索に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        "register entity" => {
            match register_entity(connection, state.clone()) {
                Ok(state) => {
                    println!("CentralサーバーへのEntityの登録に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("CentralサーバーへのEntityの登録に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        "register role" => {
            match register_role(connection, state.clone()) {
                Ok(state) => {
                    println!("CentralサーバーへのRoleの登録に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("CentralサーバーへのRoleの登録に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        "register user" => {
            match register_user(connection, state.clone()) {
                Ok(state) => {
                    println!("CentralサーバーへのUserの登録に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("CentralサーバーへのUserの登録に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        "execute command" => {
            match execute_command(connection, state.clone()) {
                Ok(state) => {
                    println!("リモートでのコマンドの実行に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("リモートでのコマンドの実行に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        "generate key" => {
            Ok(state)
        }
        "help" | _ => {
            println!("{}", constants::HELP_TEXT);
            Ok(state)
        }
    }
}