use anyhow::{Result, anyhow};

use common::messages::VerticalMessage;
use std::convert::TryInto;
use common::db::utils::{
    establish_connection
};
use common::db::models::actor::find_actor;
use common::pki::{
    hash,
    verify,
};
use common::connection::Connection;
use common::state::StateTrait;
use super::super::state::State;
use std::time::{Instant, Duration};
use common::utils::print_time;

pub fn identificate(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::IdentificateReq1 { actor_id, signature } = data {
        
        let start = Instant::now();
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

        connection.write_message(&VerticalMessage::IdentificateRes1 {
            status: String::from("OK")
        })?;

        let state = State::new(
            Some(actor),
            state.box_secret_key().clone(),
            state.box_public_key().clone(),
            state.box_certificate().clone(),
        );

        let duration = start.elapsed();
        print_time(duration);
        return Ok(state);
    } else {
        return Err(anyhow!("IdentificateReq1が渡されませんでした"));
    }
}