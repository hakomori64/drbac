use anyhow::{Result, anyhow};
use common::messages::VerticalMessage;
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
use common::state::StateTrait;
use super::super::state::State;
use std::time::{Instant,Duration};
use common::utils::print_time;


pub fn register_entity(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    let start = Instant::now();
    if let VerticalMessage::RegisterEntityReq1 { name, public_key } = data {
        let conn = establish_connection()?;
        let actor_id = generate_actor_id()?;
        create_entity(&conn, actor_id.clone(), name, None, Some(public_key))?;
        let entity = find_actor(&conn, actor_id)?;

        connection.write_message(&VerticalMessage::RegisterEntityRes1 {
            entity,
            certificate: state.box_certificate().unwrap()
        })?;

        let duration = start.elapsed();
        print_time(duration);
        Ok(state)
    } else {
        return Err(anyhow!("RegisterEntityReq1でないリクエストを受け取りました"));
    }
}

pub fn register_role(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    let start = Instant::now();
    if let VerticalMessage::RegisterRoleReq1 { name, is_assignment, public_key } = data {
        // check if state.actor is entity
        if state.opponent_actor.clone().is_none() {
            return Err(anyhow!("認証が終わっていません"));
        }

        let entity_id = if let Actor::Entity {actor_id, ..} = state.opponent_actor.clone().unwrap() { actor_id } else {
            return Err(anyhow!("Entityとして認証されていません"));
        };

        let conn = establish_connection()?;
        let actor_id = generate_actor_id()?;
        create_role(&conn, actor_id.clone(), entity_id, name, is_assignment, None, Some(public_key))?;
        let role = find_actor(&conn, actor_id)?;

        connection.write_message(&VerticalMessage::RegisterRoleRes1 {
            role
        })?;

        let duration = start.elapsed();
        print_time(duration);
        Ok(state)
    } else {
        return Err(anyhow!("RegisterRoleReq1でないリクエストを受け取りました"));
    }
}

pub fn register_user(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    let start = Instant::now();
    if let VerticalMessage::RegisterUserReq1 { name, public_key } = data {
        // check if state.actor is entity
        if state.opponent_actor.clone().is_none() {
            return Err(anyhow!("認証が終わっていません"));
        }

        let entity_id = if let Actor::Entity {actor_id, ..} = state.opponent_actor.clone().unwrap() { actor_id } else {
            return Err(anyhow!("Entityとして認証されていません"));
        };

        let conn = establish_connection()?;
        let actor_id = generate_actor_id()?;
        create_user(&conn, actor_id.clone(), entity_id, name, None, Some(public_key))?;
        let user = find_actor(&conn, actor_id)?;

        connection.write_message(&VerticalMessage::RegisterUserRes1 {
            user
        })?;
        let duration = start.elapsed();
        print_time(duration);
        Ok(state)
    } else {
        return Err(anyhow!("RegisterUserReq1でないリクエストを受け取りました"));
    }
}