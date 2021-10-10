use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use crate::schema::delegations;
use crate::db::models::actor::Actor;
use crate::db::models::actor;
use anyhow::{Result, anyhow};

#[derive(Identifiable, Queryable)]
#[table_name = "delegations"]
pub struct RawDelegation {
    pub id: i32,
    pub subject_id: String,
    pub object_id: String,
    pub issuer_id: String
}

#[derive(Debug)]
pub struct Delegation {
    pub id: i32,
    pub subject: Actor,
    pub object: Actor,
    pub issuer: Actor,
}

#[derive(Insertable)]
#[table_name = "delegations"]
pub struct NewDelegation {
    pub subject_id: String,
    pub object_id: String,
    pub issuer_id: String,
}

pub fn create_delegation(conn: &SqliteConnection, subject: &Actor, object: &Actor, issuer: &Actor) -> Result<usize> {
    let new_delegation = NewDelegation {
        subject_id: subject.actor_id(),
        object_id: object.actor_id(),
        issuer_id: issuer.actor_id(),
    };

    diesel::insert_into(delegations::table)
        .values(&new_delegation)
        .execute(conn)
        .map_err(|_| anyhow!("delegationの作成に失敗しました"))
}

pub fn get_delegations(conn: &SqliteConnection) -> Result<Vec<Delegation>> {
    use crate::schema::delegations::dsl::{delegations};

    let results = delegations.load::<RawDelegation>(conn)?;

    let mut dlgs: Vec<Delegation> = vec![];

    for raw_delegation in results {
        dlgs.push(Delegation {
            id: raw_delegation.id,
            subject: actor::find_actor(conn, raw_delegation.subject_id)?,
            object: actor::find_actor(conn, raw_delegation.object_id)?,
            issuer: actor::find_actor(conn, raw_delegation.issuer_id)?
        });
    }

    Ok(dlgs)
}

pub fn filter_delegations(
    conn: &SqliteConnection,
    subject: Option<Actor>,
    object: Option<Actor>,
    issuer: Option<Actor>
) -> Result<Vec<Delegation>> {
    let mut query = delegations::table.order(delegations::id.desc()).into_boxed();

    if let Some(x) = subject {
        query = query.filter(delegations::subject_id.eq(x.actor_id()));
    }

    if let Some(x) = object {
        query = query.filter(delegations::object_id.eq(x.actor_id()));
    }

    if let Some(x) = issuer {
        query = query.filter(delegations::issuer_id.eq(x.actor_id()));
    }

    let results = query.load::<RawDelegation>(conn)?;

    let mut dlgs: Vec<Delegation> = vec![];

    for raw_delegation in results {
        dlgs.push(Delegation {
            id: raw_delegation.id,
            subject: actor::find_actor(conn, raw_delegation.subject_id)?,
            object: actor::find_actor(conn, raw_delegation.object_id)?,
            issuer: actor::find_actor(conn, raw_delegation.issuer_id)?
        });
    }

    Ok(dlgs)
}

pub fn validate_delegation(conn: &SqliteConnection, _subject: &Actor, object: &Actor, issuer: &Actor) -> Result<()> {
    // subject, issuerの種類は(entity | role | user)のどれか
    // objectの種類はrole
    // issuerがロールの付与権限を持っていたら認める
    if let &Actor::Role { .. } = object {} else {
        return Err(anyhow!("objectのタイプがRoleではありません"));
    }

    if let Actor::Role {..} = issuer {
        return Err(anyhow!("issuerがロールであってはいけません"));
    }

    if let Actor::Entity { actor_id, .. } = issuer.clone() {
        if let Actor::Role { entity_id, .. } = object.clone() {
            let parent = actor::find_actor(conn, entity_id)?;
            if actor_id == parent.actor_id() {
                return Ok(());
            }
        }
    }
    
    // issuerがobjectの付与権限object(is_assignment = true)を持っているかを確認する
    let issuer_roles = get_roles(conn, issuer)?;

    if issuer_roles.iter().cloned().filter(|x| {

        if let Actor::Role { name, is_assignment, ..} = x.clone() {
            object.name() == name && is_assignment == true
        } else {
            false
        }
    }).collect::<Vec<Actor>>().len() == 1 {
            return Ok(());
    }

    Err(anyhow!("不正な付与です"))
}

pub fn get_roles(conn: &SqliteConnection, subject: &Actor) -> Result<Vec<Actor>> {
    // subjectに付与されているロールを再帰的に探す
    let mut roles = vec![];
    let mut candidates = std::collections::VecDeque::new();
    candidates.push_back(subject.clone());
    while ! candidates.is_empty() {
        let object = candidates.pop_front().unwrap();
        for delegation in filter_delegations(conn, Some(object.clone()), None, None)? {
            if ! roles.contains(&delegation.object) {
                roles.push(delegation.object.clone());
                candidates.push_back(delegation.object.clone());
            }
        }
    }

    Ok(roles)
}