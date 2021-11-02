use anyhow::{Result};
use common::messages::HorizontalMessage;
use common::connection::Connection;
use super::state::State;

pub fn handle_request(_connection: &mut Connection, state: State, message: HorizontalMessage) -> Result<State> {
    match message {
        _ => {
            Ok(state)
        }
    }
}