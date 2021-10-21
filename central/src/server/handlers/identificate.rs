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
    if let Message::IdentificateReq1 { actor_id, signature } = data {
        
        let conn = establish_connection()?;
        let actor = find_actor(&conn, actor_id.clone())?;
        let public_key_content = actor.public_key();
        if public_key_content.is_none() {
            return Err(anyhow!("public_keyが登録されていません"));
        }
        let public_key_content = public_key_content.unwrap();
        let public_key_blob = hash(&public_key_content)?;
        
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

        connection.write_message(&Message::IdentificateRes1 {
            server_signature: server_signature.to_vec()
        })?;

        match connection.read_message()? {
            Message::IdentificateReq2 {..} => {},
            _ => return Err(anyhow!("IdentiricateReq2でないリクエストを受け取りました"))
        }

        let common_key = rand::thread_rng().gen::<[u8; 32]>();
        let key = &common_key;
        let aes = AES::new(key);

        connection.write_message(&Message::IdentificateRes2 {
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