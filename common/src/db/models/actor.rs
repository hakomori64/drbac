use serde::{Serialize, Deserialize};
use diesel::sqlite::SqliteConnection;
use anyhow::{Result, anyhow};
use regex::Regex;

use super::entity::{
    get_entities
};
use super::role::{
    get_roles
};
use super::user::{
    get_users
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Actor {
    Entity {
        id: i32,
        actor_id: String,
        name: String,
        secret_key: Option<Vec<u8>>,
        public_key: Option<Vec<u8>>,
    },
    Role {
        id: i32,
        actor_id: String,
        entity_id: String,
        name: String,
        is_assignment: bool,
        secret_key: Option<Vec<u8>>,
        public_key: Option<Vec<u8>>,
    },
    User {
        id: i32,
        actor_id: String,
        entity_id: String,
        name: String,
        secret_key: Option<Vec<u8>>,
        public_key: Option<Vec<u8>>,
    }
}

impl Actor {
    pub fn actor_id(&self) -> String {
        match self {
            Actor::Entity { actor_id, .. } => actor_id,
            Actor::Role { actor_id, .. } => actor_id,
            Actor::User { actor_id, .. } => actor_id,
        }.clone()
    }

    pub fn name(&self) -> String {
        match self {
            Actor::Entity { name, .. } => name,
            Actor::Role { name, .. } => name,
            Actor::User { name, .. } => name,
        }.clone()
    }

    pub fn secret_key(&self) -> Option<Vec<u8>> {
        match self {
            Actor::Entity { secret_key, .. } => secret_key.clone(),
            Actor::Role { secret_key, .. } => secret_key.clone(),
            Actor::User { secret_key, .. } => secret_key.clone(),
        }
    }

    pub fn public_key(&self) -> Option<Vec<u8>> {
        match self {
            Actor::Entity { public_key, .. } => public_key.clone(),
            Actor::Role { public_key, .. } => public_key.clone(),
            Actor::User { public_key, .. } => public_key.clone(),
        }
    }
}

pub fn get_actors(
    conn: &SqliteConnection
) -> Result<Vec<Actor>> {
    let mut results: Vec<Actor> = vec![];

    let mut entities = get_entities(
        conn,
        None,
        None,
        None,
        None
    )?;

    let mut roles = get_roles(
        conn,
        None,
        None,
        None,
        None,
        None,
        None
    )?;

    let mut users = get_users(
        conn,
        None,
        None,
        None,
        None,
        None,
    )?;
    
    results.append(&mut entities);
    results.append(&mut roles);
    results.append(&mut users);

    Ok(results)
}

pub fn find_actor(
    conn: &SqliteConnection,
    actor_id: String
) -> Result<Actor> {
    let mut results: Vec<Actor> = vec![];

    let mut entities = get_entities(
        conn,
        Some(actor_id.clone()),
        None,
        None,
        None
    )?;

    let mut roles = get_roles(
        conn,
        Some(actor_id.clone()),
        None,
        None,
        None,
        None,
        None
    )?;

    let mut users = get_users(
        conn,
        Some(actor_id.clone()),
        None,
        None,
        None,
        None,
    )?;
    
    results.append(&mut entities);
    results.append(&mut roles);
    results.append(&mut users);

    match results.len() {
        1 => Ok(results[0].clone()),
        _ => Err(anyhow!("actorが見つかりませんでした"))
    }
}

pub fn is_valid_actor_id_format(actor_id: &String) -> bool {
    let re = Regex::new(r#"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"#).unwrap();
    re.is_match(actor_id.as_str())
}