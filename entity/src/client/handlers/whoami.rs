use anyhow::{Result, anyhow};

use common::connection::Connection;
use super::super::state::State;
use common::messages::VerticalMessage;

pub fn whoami(connection: &mut Connection, state: State) -> Result<State> {

    connection.write_message(&VerticalMessage::WhoamiReq1 {})?;
    let (actor, public_key_blob) = match connection.read_message()? {
        VerticalMessage::WhoamiRes1 { actor, public_key_blob } => (actor, public_key_blob),
        _ => return Err(anyhow!("WhoamiRes1ではないメッセージを受信しました"))
    };

    println!("You are {}", actor.name());
    println!("Your actor type is {:?}", actor);
    println!("public key blob: {:?}", public_key_blob);

    assert_eq!(state.actor().is_some(), true);

    Ok(state)
}