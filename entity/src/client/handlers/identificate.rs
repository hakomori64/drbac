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
    hash
};
use std::str::FromStr;
use common::messages::Message;
use super::super::state::State;

pub fn identificate(connection: &mut Connection, state: State) -> Result<State> {

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

    let (_, public_key_content) = get_key_contents(&actor, &name)?;

    let public_key_blob = hash(public_key_content)?;

    connection.write_message(&Message::IdentificateReq1 {
        name,
        actor_type,
        public_key_blob
    })?;

    let message = connection.read_json::<Message>()?;
    println!("{:?}", message);
    Ok(state)
}