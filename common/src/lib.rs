pub mod stream;
pub mod encoding;
pub mod io;
pub mod crypto;
pub mod connection;
pub mod messages;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
