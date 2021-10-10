use anyhow::{Result, anyhow};

use common::messages::Message;
use common::connection::Connection;
use common::db::models::actor::find_actor;
use common::db::utils::establish_connection;
use common::db::models::delegation::{
    create_delegation,
    validate_delegation,
};
use super::super::state::State;


pub fn delegate_role(connection: &mut Connection, state: State, data: Message) -> Result<State> {
    if let Message::DelegateRoleReq1 { subject_id, object_id, issuer_id } = data {
        let conn = establish_connection()?;
        let subject = find_actor(&conn, subject_id.clone())?;
        let object = find_actor(&conn, object_id.clone())?;
        let issuer = find_actor(&conn, issuer_id.clone())?;

        if validate_delegation(&conn, &subject, &object, &issuer).is_err() {
            return Err(anyhow!("付与が有効ではありません"));
        }

        create_delegation(&conn, &subject, &object, &issuer)?;

        connection.write_message(&Message::DelegateRoleRes1 {
            subject_id,
            object_id,
            issuer_id,
        })?;

        Ok(state)
    } else {
        return Err(anyhow!("DelegateRoleReq1でないリクエストを受け取りました"));
    }
}