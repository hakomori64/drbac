#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod stream;
pub mod encoding;
pub mod io;
pub mod crypto;
pub mod connection;
pub mod messages;
pub mod pki;
pub mod db;
pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
