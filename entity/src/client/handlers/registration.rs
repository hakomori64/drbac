use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::messages::Message;
use common::db::utils::establish_connection;
use common::db::models::actor::Actor;
use common::db::models::entity::create_entity;
use common::db::models::role::create_role;
use common::db::models::user::create_user;
use common::db::models::entity_central_relation::create_relation;
use super::super::state::State;
use common::pki::generate_key_pair;

pub fn register_entity(connection: &mut Connection, state: State) -> Result<State> {
    // name
    let name: String = io::read_until(
        "entity名を入力してください: ",
        "正しい名前を入力してください",
        |_| true
    );

    let (secret_key, public_key) = generate_key_pair()?;

    connection.write_message(&Message::RegisterEntityReq1 {
        name,
        public_key: public_key.clone()
    })?;

    match connection.read_message()? {
        // TODO central_public_keyを使ってサーバーを信頼するかどうか決める
        Message::RegisterEntityRes1 { entity, central_public_key } => {
            // 受け取った内容をローカルのDBに登録する
            let conn = establish_connection()?;
            create_entity(&conn, entity.actor_id(), entity.name(), Some(secret_key), Some(public_key))?;
            create_relation(&conn, entity.actor_id(), central_public_key)?;
            Ok(state)
        }
        _ => {
            return Err(anyhow!("RegisterEntityRes1でないレスポンスを受け取りました"));
        }
    }
}

pub fn register_role(connection: &mut Connection, state: State) -> Result<State> {
    let name: String = io::read_until(
        "role名を入力してください: ",
        "正しい名前を入力してください",
        |_| true
    );

    let is_assignment: bool = io::read_until(
        "指定した名前のロールに対する付与権限を与えるロールを作成する場合はtrueをそれ以外はfalseを入力してください",
        "true, falseのどちらかを入力してください",
        |_| true
    );

    let (secret_key, public_key) = generate_key_pair()?;

    connection.write_message(&Message::RegisterRoleReq1 {
        name,
        is_assignment,
        public_key: public_key.clone()
    })?;

    match connection.read_message()? {
        Message::RegisterRoleRes1 { role } => {
            let conn = establish_connection()?;
            if let Actor::Role{
                actor_id,
                entity_id,
                name,
                is_assignment,
                ..
            } = role {
                create_role(
                    &conn,
                    actor_id,
                    entity_id,
                    name,
                    is_assignment,
                    Some(secret_key),
                    Some(public_key)
                )?;
            } else {
                return Err(anyhow!("RegisterRoleRes1の中身がおかしいです"));
            }
            Ok(state)
        }
        _ => {
            return Err(anyhow!("RegisterRoleRes1でないレスポンスを受け取りました"));
        }
    }
}

pub fn register_user(connection: &mut Connection, state: State) -> Result<State> {
    let name: String = io::read_until(
        "user名を入力してください: ",
        "正しい名前を入力してください",
        |_| true
    );

    let (secret_key, public_key) = generate_key_pair()?;

    connection.write_message(&Message::RegisterUserReq1 {
        name,
        public_key: public_key.clone()
    })?;

    match connection.read_message()? {
        Message::RegisterUserRes1 { user } => {
            let conn = establish_connection()?;
            if let Actor::User{
                actor_id,
                entity_id,
                name,
                ..
            } = user {
                create_user(
                    &conn,
                    actor_id,
                    entity_id,
                    name,
                    Some(secret_key),
                    Some(public_key)
                )?;
            } else {
                return Err(anyhow!("RegisterUserRes1の中身がおかしいです"));
            }
            Ok(state)
        }
        _ => {
            return Err(anyhow!("RegisterUserRes1でないレスポンスを受け取りました"));
        }
    }
}