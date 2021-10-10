use crate::schema::users;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use anyhow::{Result, anyhow};

use super::actor::Actor;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub actor_id: String,
    pub entity_id: String,
    pub name: String,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

impl User {
    pub fn to_actor(&self) -> Actor {
        Actor::User {
            id: self.id,
            actor_id: self.actor_id.clone(),
            entity_id: self.entity_id.clone(),
            name: self.name.clone(),
            secret_key: self.secret_key.clone(),
            public_key: self.public_key.clone(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub actor_id: String,
    pub entity_id: String,
    pub name: String,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

pub fn create_user(
    conn: &SqliteConnection,
    actor_id: String,
    entity_id: String,
    name: String,
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> Result<()> {
    let new_user = NewUser {
        actor_id,
        entity_id,
        name,
        secret_key,
        public_key
    };

    match diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn) {
        Ok(_) => Ok(()),
        _ => Err(anyhow!("userの作成に失敗しました"))
    }
}

pub fn get_users(
    conn: &SqliteConnection,
    actor_id: Option<String>,
    entity_id: Option<String>,
    name: Option<String>,
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> Result<Vec<Actor>> {
    let mut query = users::table.order(users::id.desc()).into_boxed();

    if let Some(x) = actor_id {
        query = query.filter(users::actor_id.eq(x));
    }

    if let Some(x) = entity_id {
        query = query.filter(users::entity_id.eq(x));
    }

    if let Some(x) = name {
        query = query.filter(users::name.eq(x));
    }

    if let Some(x) = secret_key {
        query = query.filter(users::secret_key.eq(x));
    }

    if let Some(x) = public_key {
        query = query.filter(users::public_key.eq(x));
    }

    Ok(query.load::<User>(conn).map_err(|_| anyhow!("userの読み込みに失敗しました"))?
    .iter()
    .map(|user| user.to_actor()).collect())
}