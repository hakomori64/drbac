use anyhow::{Result, anyhow};

use common::io;
use common::connection::Connection;
use common::actors::Actor;
use common::actors::utils::{
    is_valid_format,
    is_valid_actor_type_str,
    get_key_contents,
};
use common::pki::{
    hash,
    sign
};
use common::crypto::aes::AES;
use std::str::FromStr;
use common::messages::Message;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, _state: State) -> Result<State> {

    let actor_type: String = io::read_until(
        "actor_type: (entity | role | user) = ",
        "正しいactor名を入力してください",
        |val| is_valid_actor_type_str(&val)
    );
    let actor = Actor::from_str(&actor_type)?;

    let name: String = io::read_until(
        "name: (String) = ",
        "名前の形式が正しくありません。<Entity>(.<Role|User>)の形式で入力してください",
        |val| is_valid_format(&actor, &val)
    );

    if !is_valid_format(&actor, &name) {
        return Err(anyhow!("正しい形式ではありません"));
    }

    let (secret_key_content, public_key_content) = get_key_contents(&actor, &name)?;

    let public_key_blob = hash(&public_key_content)?;

    connection.write_message(&Message::IdentificateReq1 {
        name: name.clone(),
        actor_type: actor_type.clone(),
        public_key_blob: public_key_blob.clone()
    })?;

    let message = connection.read_json::<Message>()?;

    if let Message::IdentificateRes1{..} = message {} else {
        return Err(anyhow!("IdentificateRes1を受け取れませんでした"));
    }

    let message = [name.as_bytes(), actor_type.as_bytes(), &public_key_blob].concat();

    let signature = sign(&message, &secret_key_content)?;

    connection.write_message(&Message::IdentificateReq2 {
        name: name.clone(),
        actor_type: actor_type.clone(),
        signature: signature.to_vec()
    })?;

    let common_key = match connection.read_message()? {
        Message::IdentificateRes2 {common_key} => common_key,
        _ => return Err(anyhow!("IdentificateRes2を受け取れませんでした"))
    };

    let key = &common_key;
    let aes = AES::new(key);

    if connection.set_crypto_module(Box::new(aes)).is_err() {
        return Err(anyhow!("暗号化モジュールの更新に失敗しました"));
    }

    let state = State::new(
        Some(name),
        Some(actor),
        Some(secret_key_content)
    );

    Ok(state)
}