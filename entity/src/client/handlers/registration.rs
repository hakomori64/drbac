use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::messages::Message;
use common::db::utils::establish_connection;
use common::db::models::entity::create_entity;
use common::db::models::entity_central_relation::create_relation;
use super::super::state::State;
use common::pki::generate_key_pair;

pub fn register_entity(connection: &mut Connection, state: State) -> Result<State> {
    // name
    let name: String = io::read_until(
        "entity名を入力してください",
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