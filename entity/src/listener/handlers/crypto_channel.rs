use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;

use common::crypto::aes::AES;
use common::messages::HorizontalMessage;
use common::connection::Connection;
use super::super::state::State;

pub fn crypto_channel(connection: &mut Connection, state: State, data: HorizontalMessage) -> Result<State> {
    if let HorizontalMessage::CryptoChannelReq1 { public_key } = data {

        let secret = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&secret);
        
        connection.write_message(&HorizontalMessage::CryptoChannelRes1 {
            public_key: public.to_bytes()
        })?;
        
        let key = secret.diffie_hellman(&PublicKey::from(public_key));
        let key = &key.as_bytes()[0..32];
        let aes = AES::new(key);
        if connection.set_crypto_module(Box::new(aes)).is_err() {
            return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
        }

        let message = connection.read_message()?;
        if let HorizontalMessage::CryptoChannelReq2 { .. } = message {} else {
            return Err(anyhow!("CRYPTO_CHANNEL_REQ2でないリクエストが来ました"));
        }
        
        connection.write_message(&HorizontalMessage::CryptoChannelRes2 {
            ping: String::from("hello")
        })?;
        
        return Ok(state);
    } else {
        return Err(anyhow!("CryptoChannelReq1が渡されませんでした"));
    }
}