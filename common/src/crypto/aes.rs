use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, NewAead, generic_array::GenericArray},
};
use anyhow::{Result, anyhow};
use rand::Rng;

use super::traits::{CommonKeyCrypto};


pub struct AES {
    key: Vec<u8>
}

impl AES {
    pub fn new(org_key: &[u8]) -> AES {
        assert_eq!(org_key.len(), 32);
        let mut key = Vec::new();
        key.extend_from_slice(org_key);
        AES { key }
    }
}

impl CommonKeyCrypto for AES {
    
    fn cipher_message(&self, plain_message: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = GenericArray::from_slice(&self.key[0..32]);

        let nonce: [u8; 12] = rand::thread_rng().gen::<[u8; 12]>();
        let nonce = GenericArray::from_slice(&nonce[0..12]);

        let cipher = Aes256Gcm::new(key);
        match cipher.encrypt(nonce, plain_message.as_ref()) {
            Ok(data) => Ok((data, nonce.to_vec())),
            Err(_) => Err(anyhow!("cannot encrypt data"))
        }
    }
    
    fn decipher_message(&self, cipher_message: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        
        let key = GenericArray::from_slice(&self.key);
        let nonce = GenericArray::from_slice(nonce);

        let cipher = Aes256Gcm::new(key);
        
        match cipher.decrypt(nonce, cipher_message) {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("cannot decrypt data"))
        }
    }
}