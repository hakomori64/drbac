use std::net::{
    TcpStream,
    TcpListener,
};

use crate::thread_pool::ThreadPool;

pub struct SocketServer<'a> {
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

    pub fn start<F>(&self, handler: F)
    where F: Fn(TcpStream) -> () + Send + 'static + Copy
    {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        let pool = ThreadPool::new(5);
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(move || {
                handler(stream);
            });
        }
    }
}