use anyhow::{Result, anyhow};

use common::messages::VerticalMessage;
use common::connection::Connection;
use common::db::models::actor::find_actor;
use common::db::utils::establish_connection;
use common::db::models::delegation::{
    create_delegation,
    validate_delegation,
    get_roles,
};
use super::super::state::State;
use std::time::{Instant,Duration};
use common::utils::print_time;

pub fn delegate_role(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    let start = Instant::now();
    if let VerticalMessage::DelegateRoleReq1 { subject_id, object_id, issuer_id } = data {
        let conn = establish_connection()?;
        let subject = find_actor(&conn, subject_id.clone())?;
        let object = find_actor(&conn, object_id.clone())?;
        let issuer = find_actor(&conn, issuer_id.clone())?;

        if validate_delegation(&conn, &subject, &object, &issuer).is_err() {
            return Err(anyhow!("付与が有効ではありません"));
        }

        create_delegation(&conn, &subject, &object, &issuer)?;

        connection.write_message(&VerticalMessage::DelegateRoleRes1 {
            subject_id,
            object_id,
            issuer_id,
        })?;

        let duration = start.elapsed();
        print_time(duration);
        Ok(state)
    } else {
        return Err(anyhow!("DelegateRoleReq1でないリクエストを受け取りました"));
    }
}

pub fn search_roles(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::SearchRolesReq1 { subject_id } = data {
        let conn = establish_connection()?;
        let subject = find_actor(&conn, subject_id)?;
        let roles = get_roles(&conn, &subject)?;

        connection.write_message(&VerticalMessage::SearchRolesRes1 {
            roles
        })?;
        Ok(state)
    } else {
        return Err(anyhow!("SearchRolesReq1でないリクエストを受け取りました"));
    }
}