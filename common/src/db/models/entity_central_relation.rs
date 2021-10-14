use crate::schema::entity_central_relation;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use anyhow::{Result, anyhow};

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "entity_central_relation"]
pub struct EntityCentralRelation {
    pub id: i32,
    pub entity_id: String,
    pub central_key: Vec<u8>
}

#[derive(Insertable)]
#[table_name = "entity_central_relation"]
pub struct NewEntityCentralRelation {
    pub entity_id: String,
    pub central_key: Vec<u8>
}

pub fn create_relation(
    conn: &SqliteConnection,
    entity_id: String,
    central_key: Vec<u8>
) -> Result<()> {
    let new_relation = NewEntityCentralRelation {
        entity_id,
        central_key,
    };

    match diesel::insert_into(entity_central_relation::table)
            .values(&new_relation)
            .execute(conn) {
        
        Ok(_) => Ok(()),
        _ => Err(anyhow!("relationの作成に失敗しました"))
    }
}

pub fn get_relation(
    conn: &SqliteConnection,
    entity_id: Option<String>,
    central_key: Option<Vec<u8>>
) -> Result<Vec<EntityCentralRelation>> {
    let mut query = entity_central_relation::table.order(entity_central_relation::id.desc()).into_boxed();

    if let Some(x) = entity_id {
        query = query.filter(entity_central_relation::entity_id.eq(x));
    }

    if let Some(x) = central_key {
        query = query.filter(entity_central_relation::central_key.eq(x));
    }

    Ok(query.load::<EntityCentralRelation>(conn).map_err(|_| anyhow!("entity_central_relationの読み込みに失敗しました"))?)
}