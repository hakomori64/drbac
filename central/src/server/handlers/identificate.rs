use anyhow::{Result, anyhow};

use common::messages::Message;
use std::convert::TryInto;
use common::actor_type::utils::{
    get_public_key_path,
    get_key_content,
};
use common::db::utils::{
    establish_connection
};
use common::crypto::aes::AES;
use common::pki::{
    hash,
    verify,
};
use rand::Rng;
use common::connection::Connection;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, _state: State, data: Message) -> Result<State> {
    if let Message::IdentificateReq1 { name, actor_type, public_key_blob } = data {

        let public_key_path = get_public_key_path(&actor_type, &name)?;
        let public_key_content = get_key_content(public_key_path)?;
        let local_public_key_blob = hash(&public_key_content)?;
        
        if public_key_blob != local_public_key_blob {
            connection.write_message(&Message::Error {
                reason: String::from("public_key_blobが一致しません")
            })?;
            return Err(anyhow!("public key blobが一致しません"));
        }

        connection.write_message(&Message::IdentificateRes1 {})?;

        let message = connection.read_message()?;
        let (second_name, second_actor_type, signature) = match message {
            Message::IdentificateReq2 { name, actor_type, signature } => (name, actor_type, signature),
            _ => return Err(anyhow!("IdentificateReq2以外が渡されました"))
        };

        if name != second_name || actor_type != second_actor_type {
            return Err(anyhow!("一回目に送られてきたリクエストと整合が取れません"));
        }

        let message = [name.as_bytes(), &(actor_type as i32).to_ne_bytes(), &public_key_blob].concat();

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

        let common_key = rand::thread_rng().gen::<[u8; 32]>();
        let key = &common_key;
        let aes = AES::new(key);

        let conn = match establish_connection() {
            Ok(conn) => conn,
            Err(_) => return Err(anyhow!("コネクションの確立に失敗しました"))
        };
        println!("here");
        let actor = match get_actor_by_name(
            &conn,
            name.as_str()) {
            Ok(actor) => actor,
            Err(_) => return Err(anyhow!("actorの取得に失敗しました"))
        };
        connection.write_message(&Message::IdentificateRes2 {
            actor: actor.clone(),
            common_key: common_key
        })?;
        if connection.set_crypto_module(Box::new(aes)).is_err() {
            return Err(anyhow!("暗号化モジュールの更新に失敗しました"));
        }

        let state = State::new(
            Some(actor),
            Some(public_key_content)
        );

        return Ok(state);
    } else {
        return Err(anyhow!("IdentificateReq1が渡されませんでした"));
    }
}