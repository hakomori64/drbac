use common::connection::Connection;
use common::messages::VerticalMessage;
use anyhow::{Result, anyhow};

use super::handlers::identificate::identificate;
use super::handlers::whoami::whoami;
use super::handlers::roles::{
    delegate_role,
    search_roles,
};
use super::handlers::registration::{
    register_entity,
    register_role,
    register_user,
};
use super::state::State;


pub fn handle_request(connection: &mut Connection, state: State, message: VerticalMessage) -> Result<State> {

    match message {
        VerticalMessage::IdentificateReq1 {..} => {
            match identificate(connection, state, message) {
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
        VerticalMessage::WhoamiReq1 {..} => {
            match whoami(connection, state, message) {
                Ok(state) => {
                    println!("身分確認に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("身分確認に成功しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        VerticalMessage::DelegateRoleReq1 {..} => {
            match delegate_role(connection, state, message) {
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
        },
        VerticalMessage::SearchRolesReq1 {..} => {
            match search_roles(connection, state, message) {
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
        },
        VerticalMessage::RegisterEntityReq1 {..} => {
            match register_entity(connection, state, message) {
                Ok(state) => {
                    println!("Entityの登録に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("Entityの登録に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        VerticalMessage::RegisterRoleReq1 {..} => {
            match register_role(connection, state, message) {
                Ok(state) => {
                    println!("Roleの登録に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("Roleの登録に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        VerticalMessage::RegisterUserReq1 {..} => {
            match register_user(connection, state, message) {
                Ok(state) => {
                    println!("Userの登録に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("Userの登録に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        },
        _ => {
            return Err(anyhow!("認識できないリクエストです"));
        }
    }   
}