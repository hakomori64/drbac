use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;

use common::encoding::struct_to_value;
use common::crypto::aes::AES;
use common::messages::Message;
use common::messages::handlers::crypto_channel::*;
use common::connection::Connection;
use super::super::state::State;

pub fn crypto_channel(connection: &mut Connection, state: State, data: &CryptoChannelReq1) -> Result<State> {
    let secret = EphemeralSecret::new(OsRng);
    let public = PublicKey::from(&secret);

    let message = Message {
        header: String::from("CRYPTO_CHANNEL_RES1"),
        data: struct_to_value(CryptoChannelRes1 {
            public_key: public.to_bytes()
        }).unwrap()
    };
    connection.write_json(&message)?;

    let key = secret.diffie_hellman(&PublicKey::from(data.public_key));
    let key = &key.as_bytes()[0..32];
    let aes = AES::new(key);
    if connection.set_crypto_module(Box::new(aes)).is_err() {
        return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
    }

    let message = connection.read_json::<Message>()?;
    if message.header.as_str() != "CRYPTO_CHANNEL_REQ2" {
        return Err(anyhow!("CRYPTO_CHANNEL_REQ2でないリクエストが来ました"));
    }

    let message = Message {
        header: String::from("CRYPTO_CHANNEL_RES2"),
        data: struct_to_value(CryptoChannelRes2 {
            ping: String::from("hello")
        }).unwrap()
    };
    connection.write_json(message)?;

    Ok(state)
}