use anyhow::{Result, anyhow};
use common::messages::Message;
use common::connection::Connection;
use common::db::utils::establish_connection;
use common::db::models::entity::create_entity;
use common::db::models::actor::{
    generate_actor_id,
    find_actor,
};
use super::super::state::State;


pub fn register_entity(connection: &mut Connection, state: State, data: Message) -> Result<State> {
    if let Message::RegisterEntityReq1 { name, public_key } = data {
        let conn = establish_connection()?;
        let actor_id = generate_actor_id()?;
        create_entity(&conn, actor_id.clone(), name, None, Some(public_key))?;
        let entity = find_actor(&conn, actor_id)?;
        let publickey = state.clone().public_key;

        connection.write_message(&Message::RegisterEntityRes1 {
            entity,
            central_public_key: publickey.unwrap(),
        })?;
        Ok(state)
    } else {
        return Err(anyhow!("RegisterEntityReq1でないリクエストを受け取りました"));
    }
}