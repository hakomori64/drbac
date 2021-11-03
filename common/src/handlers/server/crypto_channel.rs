use x25519_dalek::{EphemeralSecret, PublicKey};
use anyhow::{Result, anyhow};
use rand_core::OsRng;
use base64;
use std::convert::TryInto;

use crate::crypto::aes::AES;
use crate::messages::CommonMessage;
use crate::connection::Connection;
use crate::pki::{
    parse_pem,
    verify,
    sign,
    verify_certificate,
};
use crate::state::StateTrait;
use crate::constants::CA_PUBLIC_KEY;

pub fn crypto_channel<T: StateTrait>(connection: &mut Connection, state: T) -> Result<()> {
    match connection.read_message::<CommonMessage>()? {
        CommonMessage::CryptoChannelReq1 { certificate } => {
            let secret = EphemeralSecret::new(OsRng);
            let public = PublicKey::from(&secret);

            let ca_public_key = parse_pem(String::from(CA_PUBLIC_KEY))?;
            let verified = verify_certificate(certificate.clone(), ca_public_key)?;
            if ! verified {
                return Err(anyhow!("証明書が壊れています"));
            }
            let opponent_box_public_key = base64::decode(certificate.public_key)?;

            connection.write_message(&CommonMessage::CryptoChannelRes1 {
                certificate: state.box_certificate().unwrap()
            })?;

            let public_key = match connection.read_message()? {
                CommonMessage::CryptoChannelReq2 { public_key, signature } => {
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
                _ => return Err(anyhow!("CryptoChannelReq2でないリクエストを受け取りました"))
            };
            
            let signature = sign(&public.to_bytes(), &state.box_secret_key().unwrap())?;
            connection.write_message(&CommonMessage::CryptoChannelRes2 {
                public_key: public.to_bytes(),
                signature: base64::encode(signature),
            })?;
            
            let key = secret.diffie_hellman(&PublicKey::from(public_key));
            let key = &key.as_bytes()[0..32];
            let aes = AES::new(key);
            if connection.set_crypto_module(Box::new(aes)).is_err() {
                return Err(anyhow!("暗号化モジュールの設定に失敗しました"));
            }
    
            let message = connection.read_message()?;
            if let CommonMessage::CryptoChannelReq3 { .. } = message {} else {
                return Err(anyhow!("CRYPTO_CHANNEL_REQ2でないリクエストが来ました"));
            }
            
            connection.write_message(&CommonMessage::CryptoChannelRes3 {
                ping: String::from("hello")
            })?;
            
            Ok(())

        }
        _ => {
            Err(anyhow!("CryptoChannelReq1以外のリクエストが渡されました"))
        }
    }
}