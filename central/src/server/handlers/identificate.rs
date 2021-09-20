use anyhow::{Result, anyhow};

use common::encoding::{
    struct_to_value
};
use common::messages::{
    Message,
    DataError
};
use common::actors::Actor;
use common::actors::utils::{
    get_public_key_path,
    get_key_content,
};
use common::pki::{
    hash
};
use common::messages::handlers::identificate::*;
use common::connection::Connection;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, state: State, data: &IdentificateReq1) -> Result<State> {
    let name = data.name.clone();
    let actor = Actor::from_string(&data.actor_type)?;

    let public_key_path = get_public_key_path(&actor, &name)?;
    let public_key_content = get_key_content(public_key_path)?;
    let public_key_blob = hash(public_key_content)?;

    if public_key_blob != data.public_key_blob {
        connection.write_json(&Message {
            header: String::from("IDENTIFICATE_RES1_FAILED"),
            data: struct_to_value(DataError {
                reason: String::from("public_key_blobが一致しません")
            }).unwrap()
        })?;
        return Err(anyhow!("public key blobが一致しません"));
    }
   
    Ok(state)
}