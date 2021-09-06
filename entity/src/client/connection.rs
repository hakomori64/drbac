use std::net::TcpStream;

use super::state::State;

pub fn connect(state: &mut Option<State>, host: &str, port: i32) -> Result<(), &'static str> {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(stream) => {
            *state = Some(State::new(stream));
            Ok(())
        }
        Err(_) => Err("コネクションの確立に失敗しました")
    }
}