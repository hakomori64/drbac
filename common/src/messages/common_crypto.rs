use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CommonKeyCryptoMessage {
    pub cipher_text: Vec<u8>,
    pub nonce: [u8; 12],
}

impl CommonKeyCryptoMessage {
    pub fn new(cipher_text: Vec<u8>, nonce: [u8; 12]) -> CommonKeyCryptoMessage {
        CommonKeyCryptoMessage {
            cipher_text,
            nonce
        }
    }
}