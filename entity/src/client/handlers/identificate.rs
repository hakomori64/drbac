use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::db::utils::establish_connection;
use common::db::models::actor::{
    Actor,
    find_actor
};
use common::db::models::actor::is_valid_actor_id_format;
use common::pki::{
    hash,
    sign,
};
use common::messages::VerticalMessage;
use common::state::StateTrait;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, state: State) -> Result<State> {

    let actor_id: String = io::read_until(
        "actor_id: (entity | user) = ",
        "正しいactor_idを入力してください",
        |val| is_valid_actor_id_format(val)
    );

    let conn = establish_connection()?;
    let actor = find_actor(&conn, actor_id.clone())?;

    if let Actor::Role { .. } = actor {
        return Err(anyhow!("roleで身分証明はできません"));
    }
    let secret_key_content = actor.secret_key();
    let public_key_content = actor.public_key();
    if secret_key_content.is_none() || public_key_content.is_none() {
        return Err(anyhow!("secret_key, public_keyのどちらかまたは両方が登録されていません"));
    }
    let secret_key_content = secret_key_content.unwrap();
    let public_key_content = public_key_content.unwrap();

    let public_key_blob = hash(&public_key_content)?;

    let message = [actor_id.as_bytes(), &public_key_blob].concat();

    let signature = sign(&message, &secret_key_content)?;

    connection.write_message(&VerticalMessage::IdentificateReq1 {
        actor_id: actor_id.clone(),
        signature: signature.to_vec()
    })?;

    println!("sending res2");
    let status = match connection.read_message()? {
        VerticalMessage::IdentificateRes1 { status } => status,
        _ => return Err(anyhow!("IdentificateRes2を受け取れませんでした"))
    };

    if status.as_str() != "OK" {
        return Err(anyhow!("認証に失敗しました"));
    }


    let state = State::new(
        Some(actor),
        state.box_secret_key(),
        state.box_public_key(),
        state.box_certificate(),
        state.opponent_type(),
    );

    Ok(state)
}