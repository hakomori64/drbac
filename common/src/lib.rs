#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod enums;
pub mod stream;
pub mod encoding;
pub mod io;
pub mod crypto;
pub mod connection;
pub mod messages;
pub mod pki;
pub mod db;
pub mod schema;
pub mod thread_pool;
pub mod socket_server;
pub mod handlers;
pub mod constants;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
