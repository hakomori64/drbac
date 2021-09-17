use std::net::TcpStream;
use anyhow::{Result, anyhow};

use super::super::State;
use common::connection::Connection;

pub fn connect(connection: &mut Connection, state: State, host: &str, port: i32) -> Result<State> {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(stream) => {
            connection.set_stream(stream).expect("setting stream failed");
            return Ok(state)
        },
        Err(_) => {
            return Err(anyhow!("コネクションの確立に失敗しました"));
        }
    }
}