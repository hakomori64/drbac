use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    // crypto channel
    CryptoChannelReq1 {
        public_key: [u8; 32]
    },
    CryptoChannelRes1 {
        public_key: [u8; 32]
    },
    CryptoChannelReq2 {
        ping: String
    },
    CryptoChannelRes2 {
        ping: String
    },

    // identificate
    IdentificateReq1 {
        name: String,
        actor_type: String,
        public_key_blob: Vec<u8>
    },
    IdentificateRes1 {

    },
    IdentificateReq2 {
        name: String,
        actor_type: String,
        signature: Vec<u8>
    },
    IdentificateRes2 {
        common_key: [u8; 32]
    },

    // error
    Error {
        reason: String
    }
}


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