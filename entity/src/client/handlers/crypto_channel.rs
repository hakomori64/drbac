use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;

use common::connection::Connection;
use common::crypto::aes::AES;
use common::messages::Message;
use common::encoding::{struct_to_value, value_to_struct};
use common::messages::handlers::crypto_channel::*;
use super::super::state::State;

pub fn crypto_channel(connection: &mut Connection, state: State) -> Result<State> {
    let secret = EphemeralSecret::new(OsRng);
    let public = PublicKey::from(&secret);

    let message = Message {
        req_type: String::from("CRYPTO_CHANNEL_REQ1"),
        data: struct_to_value(CryptoChannelReq1 {
            public_key: public.to_bytes()
        }).unwrap()
    };
    connection.write_json(&message)?;

    let data = match connection.read_json::<Message>() {
        Ok(message) => {
            if message.req_type.as_str() != "CRYPTO_CHANNEL_RES1" {
                return Err(anyhow!("CRYPTO_CHANNEL_RES1ではないレスポンスを受信しました"));
            }
            value_to_struct::<CryptoChannelRes1>(message.data)
        },
        Err(_) => {
            return Err(anyhow!("CRYPTO_CHANNEL_RES1のパースに失敗しました"));
        }
    }?;

    let key = secret.diffie_hellman(&PublicKey::from(data.public_key));
    let key = &key.as_bytes()[0..32];
    let aes = AES::new(key);
    if connection.set_crypto_module(Box::new(aes)).is_err() {
        return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
    }

    let message = Message {
        req_type: String::from("CRYPTO_CHANNEL_REQ2"),
        data: struct_to_value(CryptoChannelReq2 {
            ping: String::from("hello")
        }).unwrap()
    };
    connection.write_json(message)?;
    
    let message = connection.read_json::<Message>()?;
    if message.req_type.as_str() != "CRYPTO_CHANNEL_RES2" {
        return Err(anyhow!("CRYPTO_CHANNEL_RES2でないリクエストが来ました"));
    }   

    Ok(state)
}