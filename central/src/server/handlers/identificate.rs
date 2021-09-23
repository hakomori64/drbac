use anyhow::{Result, anyhow};

use common::messages::Message;
use common::actors::Actor;
use std::str::FromStr;
use common::actors::utils::{
    get_public_key_path,
    get_key_content,
};
use common::pki::{
    hash
};
use common::connection::Connection;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, state: State, data: Message) -> Result<State> {
    if let Message::IdentificateReq1 { name, actor_type, public_key_blob } = data {

        let actor = Actor::from_str(actor_type.as_str())?;
        
        let public_key_path = get_public_key_path(&actor, &name)?;
        let public_key_content = get_key_content(public_key_path)?;
        let local_public_key_blob = hash(public_key_content)?;
        
        if public_key_blob != local_public_key_blob {
            connection.write_message(&Message::Error {
                reason: String::from("public_key_blobが一致しません")
            })?;
            return Err(anyhow!("public key blobが一致しません"));
        }
        
        return Ok(state);
    } else {
        return Err(anyhow!("IdentificateReq1が渡されませんでした"));
    }
}