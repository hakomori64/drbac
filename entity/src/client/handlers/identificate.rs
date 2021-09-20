use anyhow::{Result, anyhow};

use common::io;
use common::connection::Connection;
use common::actors::Actor;
use common::actors::utils::{
    is_valid_format,
    is_valid_actor_format,
    is_valid_actor_type_str,
    get_key_paths,
    get_key_contents,
};
use common::pki::{
    hash
};
use common::messages::Message;
use common::encoding::{struct_to_value};
use common::messages::handlers::identificate::*;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, state: State) -> Result<State> {

    let actor_type: String = io::read_until(
        "actor_type: (entity | role | user) = ",
        "正しいactor名を入力してください",
        |val| is_valid_actor_type_str(&val)
    );
    let actor = Actor::from_string(&actor_type).unwrap();

    let name: String = io::read_until(
        "name: (String) = ",
        "名前の形式が正しくありません。<Entity>(.<Role|User>)の形式で入力してください",
        |val| is_valid_format(&actor, &val)
    );

    if !is_valid_format(&actor, &name) {
        return Err(anyhow!("正しい形式ではありません"));
    }

    let (secret_key_content, public_key_content) = get_key_contents(&actor, &name)?;

    let public_key_blob = hash(public_key_content)?;

    let message = Message {
        req_type: String::from("IDENTIFICATE_REQ1"),
        data: struct_to_value(IdentificateReq1 {
            name: name,
            actor_type: actor_type,
            public_key_blob: public_key_blob
        }).unwrap()
    };

    connection.write_json(&message).unwrap();

    let message = connection.read_json::<Message>()?;
    
    Ok(state)
}