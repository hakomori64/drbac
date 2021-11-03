use serde::{Serialize, Deserialize};
use crate::db::models::actor::Actor;
use crate::pki::{
    Certificate,
};

pub trait Message {}

impl Message for CommonMessage {}
impl Message for VerticalMessage {}
impl Message for HorizontalMessage {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommonMessage {
    // crypto channel
    CryptoChannelReq1 {
        certificate: Certificate,
    },
    CryptoChannelRes1 {
        certificate: Certificate,
    },
    CryptoChannelReq2 {
        public_key: [u8; 32],
        signature: String,
    },
    CryptoChannelRes2 {
        public_key: [u8; 32],
        signature: String,
    },
    CryptoChannelReq3 {
        ping: String
    },
    CryptoChannelRes3 {
        ping: String
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerticalMessage {

    // identificate
    IdentificateReq1 {
        actor_id: String,
        signature: Vec<u8>
    },
    IdentificateRes1 {
        status: String
    },

    // whoami
    WhoamiReq1 {},
    WhoamiRes1 {
        actor: Actor,
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

    RegisterEntityReq1 {
        name: String,
        public_key: Vec<u8>
    },
    RegisterEntityRes1 {
        entity: Actor,
        certificate: Certificate,
    },

    RegisterRoleReq1 {
        name: String,
        is_assignment: bool,
        public_key: Vec<u8>
    },
    RegisterRoleRes1 {
        role: Actor,
    },

    RegisterUserReq1 {
        name: String,
        public_key: Vec<u8>
    },
    RegisterUserRes1 {
        user: Actor
    },

    // error
    Error {
        reason: String
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HorizontalMessage {

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