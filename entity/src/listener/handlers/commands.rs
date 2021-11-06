use anyhow::{anyhow, Result};
use common::messages::VerticalMessage;
use common::connection::Connection;
use super::super::state::State;
use std::process::Command;
use docker_command::{
    Docker,
    RunOpt
};
use std::path::Path;

pub fn execute_command(connection: &mut Connection, state: State, data: VerticalMessage) -> Result<State> {
    if let VerticalMessage::ExecuteProxyReq1 { actor, command, args } = data {


        let output = Docker::new().run(RunOpt {
            image: "alpine:latest".into(),
            command: Some(Path::new(&command).into()),
            args: args.iter().map(|s| s.into()).collect(),
            ..Default::default()
        })
        .enable_capture()
        .run()?;
        
        connection.write_message(&VerticalMessage::ExecuteProxyRes1 {
            result: String::from_utf8(output.stdout)?
        })?;

        
        Ok(state)
    } else {
        return Err(anyhow!("ExecuteProxyReq1が渡されました"));
    }
}