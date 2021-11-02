use anyhow::{Result, anyhow};
use common::connection::Connection;
use common::messages::VerticalMessage;
use super::super::state::State;

pub fn whoami(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::WhoamiReq1 {..} = data {} else {
        return Err(anyhow!("予期しないタイプを受け取りました"));
    }

    if state.opponent_actor.is_none() {
        return Err(anyhow!("actor_typeが空です"));
    }

    connection.write_message(&VerticalMessage::WhoamiRes1 {
        actor: state.opponent_actor().unwrap(),
    })?;

    Ok(state)
}