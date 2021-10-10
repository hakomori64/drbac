use anyhow::{Result, anyhow};

use common::io;
use common::connection::Connection;
use common::actor_type::ActorType;
use common::actor_type::utils::{
    is_valid_format,
    get_key_contents,
};
use common::pki::{
    hash,
    sign
};
use common::crypto::aes::AES;
use common::messages::Message;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, _state: State) -> Result<State> {

    let actor_type: i32 = io::read_until(
        format!("actor_type: (entity ({})| role ({})| user ({})) = ",
            ActorType::Entity as i32,
            ActorType::Role as i32,
            ActorType::User as i32
        ).as_str(),
        "正しいactor名を入力してください",
        |val| ActorType::from_i32(*val).is_ok()
    );
    let actor_type = ActorType::from_i32(actor_type)?;

    let name: String = io::read_until(
        "name: (String) = ",
        "名前の形式が正しくありません。<Entity>(.<Role|User>)の形式で入力してください",
        |val| is_valid_format(&actor_type, &val)
    );

    if !is_valid_format(&actor_type, &name) {
        return Err(anyhow!("正しい形式ではありません"));
    }

    let (secret_key_content, public_key_content) = get_key_contents(&actor_type, &name)?;

    let public_key_blob = hash(&public_key_content)?;

    connection.write_message(&Message::IdentificateReq1 {
        name: name.clone(),
        actor_type: actor_type,
        public_key_blob: public_key_blob.clone()
    })?;

    let message = connection.read_json::<Message>()?;

    if let Message::IdentificateRes1{..} = message {} else {
        return Err(anyhow!("IdentificateRes1を受け取れませんでした"));
    }

    let message = [name.as_bytes(), &(actor_type as i32).to_ne_bytes(), &public_key_blob].concat();

    let signature = sign(&message, &secret_key_content)?;

    connection.write_message(&Message::IdentificateReq2 {
        name: name.clone(),
        actor_type: actor_type.clone(),
        signature: signature.to_vec()
    })?;

    // I want id, name, kind, parent_id(if exists)
    let (actor, common_key) = match connection.read_message()? {
        Message::IdentificateRes2 {actor, common_key} => (actor, common_key),
        _ => return Err(anyhow!("IdentificateRes2を受け取れませんでした"))
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