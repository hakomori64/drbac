use common::connection::Connection;
use common::messages::Message;
use anyhow::{Result, anyhow};

use super::handlers::crypto_channel::crypto_channel;
use super::handlers::identificate::identificate;
use super::handlers::whoami::whoami;
use super::handlers::roles::{
    delegate_role,
    search_roles,
};
use super::handlers::registration::{
    register_entity,
};
use super::state::State;


pub fn handle_request(connection: &mut Connection, state: State, message: Message) -> Result<State> {

    match message {
        Message::CryptoChannelReq1 {..} => {
            match crypto_channel(connection, state, message) {
                Ok(state) => {
                    println!("暗号化通信の設定に成功しました");
                    Ok(state)
                }
                Err(err) => {
                    println!("暗号化通信の設定に失敗しました");
                    println!("{}", err);
                    Err(err)
                }
            }
        }
        Message::IdentificateReq1 {..} => {
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
        Message::WhoamiReq1 {..} => {
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
        Message::DelegateRoleReq1 {..} => {
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
        Message::SearchRolesReq1 {..} => {
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
        Message::RegisterEntityReq1 {..} => {
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
        }
        _ => {
            return Err(anyhow!("認識できないリクエストです"));
        }
    }   
}