use std::net::TcpStream;

struct SocketServer<'a> {
    host: &'a str,
    port: i32
}

impl<'a> SocketServer<'a> {
    pub fn new(host: &'a str, port: i32) -> SocketServer {
        SocketServer {
            host,
            port
        }
    }

    pub fn start<F>(handler: F)
    where F: FnOnce(TcpStream) -> ()
    {

    }
}