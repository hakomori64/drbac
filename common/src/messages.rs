use serde::{Serialize, Deserialize};
use crate::db::models::actor::Actor;

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
        actor_id: String,
        public_key_blob: Vec<u8>
    },
    IdentificateRes1 {

    },
    IdentificateReq2 {
        actor_id: String,
        signature: Vec<u8>
    },
    IdentificateRes2 {
        actor: Actor,
        common_key: [u8; 32]
    },

    // whoami
    WhoamiReq1 {},
    WhoamiRes1 {
        actor: Actor,
        public_key_blob: Vec<u8>
    },

    // delegate role
    DelegateRoleReq1 {
        subject_id: String,
        object_id: String,
        issuer_id: String,
    },
    DelegateRoleRes1 {
        subject_id: String,
        object_id: String,
        issuer_id: String,
    },

    // search roles
    SearchRolesReq1 {
        subject_id: String,
    },
    SearchRolesRes1 {
        roles: Vec<Actor>
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