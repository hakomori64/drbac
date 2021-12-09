#[macro_use]
extern crate diesel;
extern crate tempfile;
extern crate nix;

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
pub mod state;
pub mod jail;
pub mod policy;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
