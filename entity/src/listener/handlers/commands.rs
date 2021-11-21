use anyhow::{anyhow, Result};
use common::messages::VerticalMessage;
use common::connection::Connection;
use super::super::state::State;
use common::jail::{
    create_directory_if_not_exists,
    assign_roles_to_guest,
    get_guest_id,
    exec_chroot,
    exec
};


pub fn execute_command(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::ExecuteProxyReq1 { actor, entity_id, command, args, roles } = data {
        let actor_id = actor.actor_id();
        assign_roles_to_guest(roles.clone(), entity_id.clone())?;
        if environment_setup(actor.actor_id(), state.enable_jail()).is_err() {
            let msg = format!("actor id {}のディレクトリのセットアップに失敗しました", actor_id);
            return Err(anyhow!(msg))
        }
        let guest_id = get_guest_id()?;

        let result = exec(command, args, guest_id)?;

        connection.write_message(&VerticalMessage::ExecuteProxyRes1 {
            result
        })?;

        loop {
            match connection.read_message::<VerticalMessage>()? {
                VerticalMessage::ExecuteProxyReq1 { command, args, .. } => {
                    // execute commands
                    let result = exec(command, args, guest_id)?;
                    connection.write_message(&VerticalMessage::ExecuteProxyRes1 {
                        result
                    })?;
                },
                VerticalMessage::ExecuteProxyReq2 { } => {
                    // close connection
                    connection.write_message(&VerticalMessage::ExecuteProxyRes2 {})?;
                    return Ok(state);
                },
                _ => {
                    return Err(anyhow!("予期しないリクエストを受け取りました"));
                }
            }
        }
    } else {
        return Err(anyhow!("ExecuteProxyReq1が渡されました"));
    }
}

fn environment_setup(actor_id: String, enable_jail: bool) -> Result<()> {
    if enable_jail {
        create_directory_if_not_exists(actor_id.as_str());
        exec_chroot(actor_id.as_str());
    }

    Ok(())
}
