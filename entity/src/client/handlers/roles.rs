use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::messages::Message;
use common::db::models::actor::is_valid_actor_id_format;
use super::super::state::State;

pub fn delegate_role(connection: &mut Connection, state: State) -> Result<State> {
    // take
    // subject, object, issuer
    let subject_id: String = io::read_until(
        "subject_id: (entity | role | user) = ",
        "正しいactor_idを入力してください",
        |val| is_valid_actor_id_format(val)
    );
    let object_id: String = io::read_until(
        "object_id: role = ",
        "正しいactor_idを入力してください",
        |val| is_valid_actor_id_format(val)
    );
    let issuer_id: String = io::read_until(
        "issuer_id: your id = ",
        "正しいactor_idを入力してください",
        |val| is_valid_actor_id_format(val)
    );

    connection.write_message(&Message::DelegateRoleReq1 {
        subject_id,
        object_id,
        issuer_id
    })?;

    match connection.read_message()? {
        Message::DelegateRoleRes1 {..} => Ok(state),
        _ => Err(anyhow!("DelegateRoleRes1でないレスポンスを受け取りました"))
    }
}