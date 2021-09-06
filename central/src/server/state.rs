use std::net::TcpStream;

pub struct State {
    pub stream: TcpStream,
}

impl State {
    /// Create new State
    /// 
    /// stream is tcp stream
    /// 
    /// # Panic
    /// 
    pub fn new(stream: TcpStream) -> State {
        State { stream }
    }
}