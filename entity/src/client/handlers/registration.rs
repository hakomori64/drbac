use common::policy::show_role_presetup_message;
use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::messages::VerticalMessage;
use common::db::utils::establish_connection;
use common::db::models::actor::Actor;
use common::db::models::entity::create_entity;
use common::db::models::role::create_role;
use common::db::models::user::create_user;
use common::db::models::entity_central_relation::create_relation;
use common::policy::{
    role_presetup,
};
use super::super::state::State;
use common::pki::{
    BoxType,
    generate_key_pair,
    parse_pem,
    verify_certificate,
};
use common::constants::CA_PUBLIC_KEY;
use std::time::{Duration,Instant};
use common::utils::print_time;

pub fn register_entity(connection: &mut Connection, state: State) -> Result<State> {
    // name
    let name: String = io::read_until(
        "entity名を入力してください: ",
        "正しい名前を入力してください",
        |_| true
    );

    let start = Instant::now();
    let (secret_key, public_key) = generate_key_pair()?;

    connection.write_message(&VerticalMessage::RegisterEntityReq1 {
        name,
        public_key: public_key.clone()
    })?;

    match connection.read_message()? {
        // TODO central_public_keyを使ってサーバーを信頼するかどうか決める
        VerticalMessage::RegisterEntityRes1 { entity, certificate } => {
            // 受け取った内容をローカルのDBに登録する
            let conn = establish_connection()?;
            let ca_public_key = parse_pem(String::from(CA_PUBLIC_KEY))?;
            let central_public_key = if verify_certificate(certificate.clone(), ca_public_key)? && certificate.box_type == BoxType::Central {
                certificate.decoded_public_key()?
            } else {
                return Err(anyhow!("送られてきた証明書に問題があります"));
            };
            
            create_entity(&conn, entity.actor_id(), entity.name(), Some(secret_key), Some(public_key))?;
            create_relation(&conn, entity.actor_id(), central_public_key)?;

            let duration = start.elapsed();
            print_time(duration);
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

    let start = Instant::now();

    let (secret_key, public_key) = generate_key_pair()?;

    connection.write_message(&VerticalMessage::RegisterRoleReq1 {
        name,
        is_assignment,
        public_key: public_key.clone()
    })?;

    match connection.read_message()? {
        VerticalMessage::RegisterRoleRes1 { role } => {
            let conn = establish_connection()?;
            if let Actor::Role{
                actor_id,
                entity_id,
                name,
                is_assignment,
                ..
            } = role {
                let role_name = format!("role-{}", &actor_id);
                role_presetup(&role_name)?;

                create_role(
                    &conn,
                    actor_id.clone(),
                    entity_id,
                    name,
                    is_assignment,
                    Some(secret_key),
                    Some(public_key)
                )?;

                show_role_presetup_message(&role_name)?;

                let duration = start.elapsed();
                print_time(duration);
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

    let start = Instant::now();
    let (secret_key, public_key) = generate_key_pair()?;

    connection.write_message(&VerticalMessage::RegisterUserReq1 {
        name,
        public_key: public_key.clone()
    })?;

    match connection.read_message()? {
        VerticalMessage::RegisterUserRes1 { user } => {
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

                let duration = start.elapsed();
                print_time(duration);
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