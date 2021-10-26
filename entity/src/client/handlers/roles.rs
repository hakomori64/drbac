use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::messages::VerticalMessage;
use common::db::models::actor::is_valid_actor_id_format;
use common::db::models::actor::Actor;
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

    connection.write_message(&VerticalMessage::DelegateRoleReq1 {
        subject_id,
        object_id,
        issuer_id
    })?;

    match connection.read_message()? {
        VerticalMessage::DelegateRoleRes1 {..} => Ok(state),
        _ => Err(anyhow!("DelegateRoleRes1でないレスポンスを受け取りました"))
    }
}

/// subject_idを入力として受け取り、そのsubjectに付属するroleの一覧を取得する
pub fn search_roles(connection: &mut Connection, state: State) -> Result<State> {
    let subject_id: String = io::read_until(
        "subject_id: (entity | role | user) = ",
        "正しいactor_idを入力してください",
        |val| is_valid_actor_id_format(val)
    );

    connection.write_message(&VerticalMessage::SearchRolesReq1 {
        subject_id: subject_id.clone()
    })?;

    match connection.read_message()? {
        VerticalMessage::SearchRolesRes1 { roles } => {
            println!("{}", format!("subject_id: {}には以下のロールが付属しています", subject_id));
            for role in roles {
                if let Actor::Role { entity_id, name, .. } = role {
                    println!("{}", format!("{}の{}", entity_id, name));
                }
            }
        }
        _ => {
            return Err(anyhow!("SearchRolesRes1でないレスポンスを受け取りました"));
        }
    }

    Ok(state)
}
