use crate::schema::entities;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use anyhow::{Result, anyhow};

use super::actor::Actor;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "entities"]
pub struct Entity {
    pub id: i32,
    pub actor_id: String,
    pub name: String,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

impl Entity {
    pub fn to_actor(&self) -> Actor {
        Actor::Entity {
            id: self.id,
            actor_id: self.actor_id.clone(),
            name: self.name.clone(),
            secret_key: self.secret_key.clone(),
            public_key: self.public_key.clone(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "entities"]
pub struct NewEntity {
    pub actor_id: String,
    pub name: String,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
}

pub fn create_entity(
    conn: &SqliteConnection,
    actor_id: String,
    name: String,
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> Result<()> {
    let new_entity = NewEntity {
        actor_id,
        name,
        secret_key,
        public_key
    };

    match diesel::insert_into(entities::table)
            .values(&new_entity)
            .execute(conn) {
        Ok(_) => Ok(()),
        _ => Err(anyhow!("entityの作成に失敗しました"))
    }
}

pub fn get_entities(
    conn: &SqliteConnection,
    actor_id: Option<String>,
    name: Option<String>,
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> Result<Vec<Actor>> {
    let mut query = entities::table.order(entities::id.desc()).into_boxed();

    if let Some(x) = actor_id {
        query = query.filter(entities::actor_id.eq(x));
    }

    if let Some(x) = name {
        query = query.filter(entities::name.eq(x));
    }

    if let Some(x) = secret_key {
        query = query.filter(entities::secret_key.eq(x));
    }

    if let Some(x) = public_key {
        query = query.filter(entities::public_key.eq(x));
    }

    Ok(query.load::<Entity>(conn).map_err(|_| anyhow!("entityの読み込みに失敗しました"))?
    .iter()
    .map(|entity| entity.to_actor()).collect())
}