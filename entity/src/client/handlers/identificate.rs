use std::convert::TryInto;
use anyhow::{Result, anyhow};
use common::io;
use common::connection::Connection;
use common::db::utils::establish_connection;
use common::db::models::actor::{
    Actor,
    find_actor
};
use common::db::models::actor::is_valid_actor_id_format;
use common::db::models::entity_central_relation::get_relation;
use common::pki::{
    hash,
    sign,
    verify,
};
use common::crypto::aes::AES;
use common::messages::Message;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, _state: State) -> Result<State> {

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

    connection.write_message(&Message::IdentificateReq1 {
        actor_id: actor_id.clone(),
        public_key_blob: public_key_blob.clone()
    })?;

    let message = connection.read_json::<Message>()?;

    if let Message::IdentificateRes1{..} = message {} else {
        return Err(anyhow!("IdentificateRes1を受け取れませんでした"));
    }

    let message = [actor_id.as_bytes(), &public_key_blob].concat();

    let signature = sign(&message, &secret_key_content)?;

    connection.write_message(&Message::IdentificateReq2 {
        actor_id: actor_id.clone(),
        signature: signature.to_vec()
    })?;

    let server_signature = match connection.read_message()? {
        Message::IdentificateRes2 {server_signature} => server_signature,
        _ => return Err(anyhow!("IdentificateRes2を受け取れませんでした"))
    };

    let server_signature: [u8; 64] = match server_signature.try_into() {
        Ok(ba) => ba,
        Err(_) => return Err(anyhow!("server_signatureの形式が正しくありません"))
    };

    let entity_id = match actor {
        Actor::Entity { actor_id, .. } => actor_id,
        Actor::Role { entity_id, .. } => entity_id,
        Actor::User { entity_id, .. } => entity_id
    };

    let relations = get_relation(&conn, Some(entity_id), None)?;
    if relations.len() != 1 {
        return Err(anyhow!("entityを複数のホストに対して登録しているか、登録された形跡がありません"));
    }

    match verify(server_signature, &message, &relations[0].central_key) {
        Err(_) => {
            return Err(anyhow!("centralの認証に失敗しました"));
        },
        _ => {}
    }

    connection.write_message(&Message::IdentificateReq3 {})?;

    let (actor, common_key) = match connection.read_message()? {
        Message::IdentificateRes3 {actor, common_key} => (actor, common_key),
        _ => return Err(anyhow!("IdentificateRes3を受け取れませんでした"))
    };

    let key = &common_key;
    let aes = AES::new(key);

    if connection.set_crypto_module(Box::new(aes)).is_err() {
        return Err(anyhow!("暗号化モジュールの更新に失敗しました"));
    }

    let state = State::new(
        Some(actor),
        Some(secret_key_content)
    );

    Ok(state)
}