use anyhow::{Result, anyhow};
use common::messages::Message;
use common::connection::Connection;
use common::db::utils::establish_connection;
use common::db::models::actor::Actor;
use common::db::models::entity::create_entity;
use common::db::models::role::create_role;
use common::db::models::user::create_user;
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

pub fn register_role(connection: &mut Connection, state: State, data: Message) -> Result<State> {
    if let Message::RegisterRoleReq1 { name, is_assignment, public_key } = data {
        // check if state.actor is entity
        if state.actor.clone().is_none() {
            return Err(anyhow!("認証が終わっていません"));
        }

        let entity_id = if let Actor::Entity {actor_id, ..} = state.actor.clone().unwrap() { actor_id } else {
            return Err(anyhow!("Entityとして認証されていません"));
        };

        let conn = establish_connection()?;
        let actor_id = generate_actor_id()?;
        create_role(&conn, actor_id.clone(), entity_id, name, is_assignment, None, Some(public_key))?;
        let role = find_actor(&conn, actor_id)?;

        connection.write_message(&Message::RegisterRoleRes1 {
            role
        })?;
        Ok(state)
    } else {
        return Err(anyhow!("RegisterRoleReq1でないリクエストを受け取りました"));
    }
}

pub fn register_user(connection: &mut Connection, state: State, data: Message) -> Result<State> {
    if let Message::RegisterUserReq1 { name, public_key } = data {
        // check if state.actor is entity
        if state.actor.clone().is_none() {
            return Err(anyhow!("認証が終わっていません"));
        }

        let entity_id = if let Actor::Entity {actor_id, ..} = state.actor.clone().unwrap() { actor_id } else {
            return Err(anyhow!("Entityとして認証されていません"));
        };

        let conn = establish_connection()?;
        let actor_id = generate_actor_id()?;
        create_user(&conn, actor_id.clone(), entity_id, name, None, Some(public_key))?;
        let user = find_actor(&conn, actor_id)?;

        connection.write_message(&Message::RegisterUserRes1 {
            user
        })?;
        Ok(state)
    } else {
        return Err(anyhow!("RegisterUserReq1でないリクエストを受け取りました"));
    }
}