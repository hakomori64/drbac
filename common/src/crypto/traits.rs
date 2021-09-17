use anyhow::Result;

pub trait CommonKeyCrypto {
    fn cipher_message(&self, plain_message: &[u8]) -> Result<(Vec<u8>, Vec<u8>)>;
    fn decipher_message(&self, encrypted_message: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}