/*
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, NewAead, generic_array::GenericArray},
};
use anyhow::{Result, anyhow};
use rand::Rng;
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

use super::traits::{CommonKeyCrypto};


pub struct DiffieHellman {
    key: Option<Vec<u8>>,
    secret: EphemeralSecret,
    pub public: PublicKey
}

impl DiffieHellman {
    pub fn new() -> DiffieHellman {
        let secret = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&secret);

        DiffieHellman {
            key: None,
            secret,
            public,
        }
    }

    pub fn key_share(&mut self, opponent_public_key: PublicKey) -> Result<()> {
        let secret : &mut EphemeralSecret = &mut self.secret;
        let key = secret.diffie_hellman(&opponent_public_key);
        let key: &[u8] = &key.as_bytes()[0..16];
        self.key = Some(key.iter().cloned().collect());
        Ok(())
    }
}

impl CommonKeyCrypto for DiffieHellman {
    
    fn cipher_message(&self, plain_message: &[u8]) -> Result<Vec<u8>> {

        if self.key.is_none() {
            return Err(anyhow!("鍵交換が完了していません！"));
        }
        
        let key = self.key.as_ref().unwrap();
        let key = GenericArray::from_slice(&key);

        let nonce = rand::thread_rng().gen::<[u8; 16]>();
        let nonce = GenericArray::from_slice(&nonce);

        let cipher = Aes256Gcm::new(key);
        match cipher.encrypt(nonce, plain_message.as_ref()) {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("cannot encrypt data"))
        }
    }
    
    fn decipher_message(&self, cipher_message: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {

        if self.key.is_none() {
            return Err(anyhow!("鍵交換が完了していません"));
        }
        
        let key = self.key.as_ref().unwrap();
        let key = GenericArray::from_slice(&key);
        let nonce = GenericArray::from_slice(nonce);

        let cipher = Aes256Gcm::new(key);
        
        match cipher.decrypt(nonce, cipher_message) {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("cannot decrypt data"))
        }
    }
}
*/