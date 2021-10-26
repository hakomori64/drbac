use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;

use crate::connection::Connection;
use crate::crypto::aes::AES;
use crate::messages::CommonMessage;
use crate::enums::ServerType;

pub fn crypto_channel(connection: &mut Connection) -> Result<ServerType> {
    let secret = EphemeralSecret::new(OsRng);
    let public = PublicKey::from(&secret);

    connection.write_message(&CommonMessage::CryptoChannelReq1 {
        public_key: public.to_bytes()
    })?;

    let message = connection.read_message()?;
    let public_key = match message {
        CommonMessage::CryptoChannelRes1 { public_key } => public_key,
        _ => return Err(anyhow!("CryptoChannelRes1のパースに失敗しました"))
    };

    let key = secret.diffie_hellman(&PublicKey::from(public_key));
    let key = &key.as_bytes()[0..32];
    let aes = AES::new(key);
    if connection.set_crypto_module(Box::new(aes)).is_err() {
        return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
    }

    connection.write_message(&CommonMessage::CryptoChannelReq2 {
        ping: String::from("hello")
    })?;
    
    let message = connection.read_message()?;
    let server_type = if let CommonMessage::CryptoChannelRes2 { server_type } = message { server_type } else {
        return Err(anyhow!("CRYPTO_CHANNEL_RES2でないリクエストが来ました"));
    };

    Ok(server_type)
}