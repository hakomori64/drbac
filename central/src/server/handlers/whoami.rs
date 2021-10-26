use anyhow::{Result, anyhow};
use common::pki::hash;
use common::connection::Connection;
use common::messages::VerticalMessage;
use super::super::state::State;

pub fn whoami(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::WhoamiReq1 {..} = data {} else {
        return Err(anyhow!("予期しないタイプを受け取りました"));
    }

    if state.actor.is_none() {
        return Err(anyhow!("actor_typeが空です"));
    }
    if state.public_key().is_none() {
        return Err(anyhow!("public keyが空です"));
    }

    connection.write_message(&VerticalMessage::WhoamiRes1 {
        actor: state.actor().unwrap(),
        public_key_blob: hash(&state.public_key().unwrap())?
    })?;

    Ok(state)
}