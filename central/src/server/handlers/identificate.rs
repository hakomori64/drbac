use anyhow::{Result, anyhow};

use common::messages::Message;
use std::convert::TryInto;
use common::db::utils::{
    establish_connection
};
use common::db::models::actor::find_actor;
use common::crypto::aes::AES;
use common::pki::{
    hash,
    sign,
    verify,
};
use rand::Rng;
use common::connection::Connection;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, state: State, data: Message) -> Result<State> {
    if let Message::IdentificateReq1 { actor_id, public_key_blob } = data {
        
        let conn = establish_connection()?;
        let actor = find_actor(&conn, actor_id.clone())?;
        let public_key_content = actor.public_key();
        if public_key_content.is_none() {
            return Err(anyhow!("public_keyが登録されていません"));
        }
        let public_key_content = public_key_content.unwrap();
        let local_public_key_blob = hash(&public_key_content)?;
        
        if public_key_blob != local_public_key_blob {
            connection.write_message(&Message::Error {
                reason: String::from("public_key_blobが一致しません")
            })?;
            return Err(anyhow!("public key blobが一致しません"));
        }

        let server_public_key_blob = hash(&state.public_key.clone().unwrap())?;

        connection.write_message(&Message::IdentificateRes1 {
            server_public_key_blob
        })?;

        let message = connection.read_message()?;
        let (second_actor_id, signature) = match message {
            Message::IdentificateReq2 { actor_id, signature } => (actor_id, signature),
            _ => return Err(anyhow!("IdentificateReq2以外が渡されました"))
        };

        if actor_id != second_actor_id {
            return Err(anyhow!("一回目に送られてきたリクエストと整合が取れません"));
        }

        let message = [actor_id.as_bytes(), &public_key_blob].concat();

        let signature: [u8; 64] = match signature.try_into() {
            Ok(ba) => ba,
            Err(_) => return Err(anyhow!("signatureの形式が正しくありません"))
        };

        match verify(signature, &message, &public_key_content) {
            Err(_) => {
                return Err(anyhow!("認証に失敗しました"));
            },
            _ => {}
        };

        let server_signature = sign(&message, &state.secret_key.clone().unwrap())?;

        connection.write_message(&Message::IdentificateRes2 {
            server_signature: server_signature.to_vec()
        })?;

        match connection.read_message()? {
            Message::IdentificateReq3 {..} => {},
            _ => return Err(anyhow!("IdentiricateReq3でないリクエストを受け取りました"))
        }

        let common_key = rand::thread_rng().gen::<[u8; 32]>();
        let key = &common_key;
        let aes = AES::new(key);

        connection.write_message(&Message::IdentificateRes3 {
            actor: actor.clone(),
            common_key: common_key
        })?;
        if connection.set_crypto_module(Box::new(aes)).is_err() {
            return Err(anyhow!("暗号化モジュールの更新に失敗しました"));
        }

        let state = State::new(
            Some(actor),
            state.secret_key.clone(),
            state.public_key.clone()
        );

        return Ok(state);
    } else {
        return Err(anyhow!("IdentificateReq1が渡されませんでした"));
    }
}