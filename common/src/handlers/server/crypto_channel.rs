use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;

use crate::crypto::aes::AES;
use crate::messages::CommonMessage;
use crate::connection::Connection;
use crate::enums::ServerType;

pub fn crypto_channel(connection: &mut Connection, server_type: ServerType) -> Result<()> {
    match connection.read_message::<CommonMessage>()? {
        CommonMessage::CryptoChannelReq1 { public_key } => {
            let secret = EphemeralSecret::new(OsRng);
            let public = PublicKey::from(&secret);
            
            connection.write_message(&CommonMessage::CryptoChannelRes1 {
                public_key: public.to_bytes()
            })?;
            
            let key = secret.diffie_hellman(&PublicKey::from(public_key));
            let key = &key.as_bytes()[0..32];
            let aes = AES::new(key);
            if connection.set_crypto_module(Box::new(aes)).is_err() {
                return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
            }
    
            let message = connection.read_message()?;
            if let CommonMessage::CryptoChannelReq2 { .. } = message {} else {
                return Err(anyhow!("CRYPTO_CHANNEL_REQ2でないリクエストが来ました"));
            }
            
            connection.write_message(&CommonMessage::CryptoChannelRes2 {
                server_type: server_type
            })?;
            
            return Ok(());

        }
        _ => {
            Err(anyhow!("CryptoChannelReq1以外のリクエストが渡されました"))
        }
    }
}