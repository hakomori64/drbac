use crate::schema::roles;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use anyhow::{Result, anyhow};

use super::actor::Actor;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub actor_id: String,
    pub entity_id: String,
    pub name: String,
    pub is_assignment: bool,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

impl Role {
    pub fn to_actor(&self) -> Actor {
        Actor::Role {
            id: self.id,
            actor_id: self.actor_id.clone(),
            entity_id: self.entity_id.clone(),
            name: self.name.clone(),
            is_assignment: self.is_assignment,
            secret_key: self.secret_key.clone(),
            public_key: self.public_key.clone(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "roles"]
pub struct NewRole {
    pub actor_id: String,
    pub entity_id: String,
    pub name: String,
    pub is_assignment: bool,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

pub fn create_role(
    conn: &SqliteConnection,
    actor_id: String,
    entity_id: String,
    name: String,
    is_assignment: bool,
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> Result<()> {
    let new_role = NewRole {
        actor_id,
        entity_id,
        name,
        is_assignment,
        secret_key,
        public_key
    };

    match diesel::insert_into(roles::table)
            .values(&new_role)
            .execute(conn) {
        Ok(_) => Ok(()),
        _ => Err(anyhow!("roleの作成に失敗しました"))
    }
}

pub fn get_roles(
    conn: &SqliteConnection,
    actor_id: Option<String>,
    entity_id: Option<String>,
    name: Option<String>,
    is_assignment: Option<bool>,
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> Result<Vec<Actor>> {
    let mut query = roles::table.order(roles::id.desc()).into_boxed();

    if let Some(x) = actor_id {
        query = query.filter(roles::actor_id.eq(x));
    }

    if let Some(x) = entity_id {
        query = query.filter(roles::entity_id.eq(x));
    }

    if let Some(x) = name {
        query = query.filter(roles::name.eq(x));
    }

    if let Some(x) = is_assignment {
        query = query.filter(roles::is_assignment.eq(x));
    }

    if let Some(x) = secret_key {
        query = query.filter(roles::secret_key.eq(x));
    }

    if let Some(x) = public_key {
        query = query.filter(roles::public_key.eq(x));
    }

    Ok(query.load::<Role>(conn).map_err(|_| anyhow!("roleの読み込みに失敗しました"))?
    .iter()
    .map(|role| role.to_actor()).collect())
}