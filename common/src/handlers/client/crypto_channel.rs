use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;
use base64;
use std::convert::TryInto;

use crate::connection::Connection;
use crate::crypto::aes::AES;
use crate::messages::CommonMessage;
use crate::pki::{
    sign,
    verify,
    parse_pem,
    verify_certificate,
    BoxType,
};
use crate::state::StateTrait;
use crate::constants::CA_PUBLIC_KEY;

pub fn crypto_channel<T: StateTrait>(connection: &mut Connection, state: T) -> Result<BoxType> {
    let secret = EphemeralSecret::new(OsRng);
    let public = PublicKey::from(&secret);

    // send own certificate
    connection.write_message(&CommonMessage::CryptoChannelReq1 {
        certificate: state.box_certificate().unwrap()
    })?;

    // receive opponent certificate
    let (opponent_box_public_key, opponent_box_type) = match connection.read_message()? {
        CommonMessage::CryptoChannelRes1 { certificate } => {
            let ca_public_key = parse_pem(String::from(CA_PUBLIC_KEY))?;
            let verified = verify_certificate(certificate.clone(), ca_public_key)?;
            if verified {
                (base64::decode(certificate.public_key)?, certificate.box_type)
            } else {
                return Err(anyhow!("送られてきた証明書の内容が壊れています"));
            }
        },
        _ => return Err(anyhow!("CryptoChannelRes1でないレスポンスを受け取りました"))
    };

    // key exchange
    let signature = sign(&public.to_bytes(), &state.box_secret_key().unwrap())?;
    connection.write_message(&CommonMessage::CryptoChannelReq2 {
        public_key: public.to_bytes(),
        signature: base64::encode(signature),
    })?;

    let message = connection.read_message()?;
    let public_key = match message {
        CommonMessage::CryptoChannelRes2 { public_key, signature } => {
            let decoded = base64::decode(signature)?;
            let decoded: [u8; 64] = match decoded.try_into() {
                Ok(ba) => ba,
                Err(_) => return Err(anyhow!("signatureの形式が正しくありません"))
            };

            match verify(decoded, &public_key, &opponent_box_public_key) {
                Ok(_) => public_key,
                Err(_) => return Err(anyhow!("相手の公開鍵のデコードに失敗しました"))
            }
        },
        _ => return Err(anyhow!("CryptoChannelRes2でないレスポンスを受け取りました"))
    };

    let key = secret.diffie_hellman(&PublicKey::from(public_key));
    let key = &key.as_bytes()[0..32];
    let aes = AES::new(key);
    if connection.set_crypto_module(Box::new(aes)).is_err() {
        return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
    }

    connection.write_message(&CommonMessage::CryptoChannelReq3 {
        ping: String::from("hello")
    })?;
    
    let message = connection.read_message()?;
    if let CommonMessage::CryptoChannelRes3 { .. } = message { } else {
        return Err(anyhow!("CRYPTO_CHANNEL_RES3でないリクエストが来ました"));
    };

    Ok(opponent_box_type)
}