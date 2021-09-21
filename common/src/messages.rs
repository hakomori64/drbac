pub mod handlers;
pub mod common_crypto;
pub mod headers;
pub mod encoding;

use  std::any::type_name;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Visitor, de::MapAccess};
use serde::ser::SerializeMap;
use serde::de::DeserializeOwned;
use strum::{EnumIter, IntoEnumIterator};

use crate::encoding::{struct_to_value, value_to_struct};
use encoding::{value_to_data};
use handlers::{
    crypto_channel,
    identificate
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataError {
    pub reason: String,
}

#[derive(Debug, EnumIter)]
pub enum Message {
    // crypto channel
    CryptoChannelReq1(crypto_channel::CryptoChannelReq1),
    CryptoChannelRes1(crypto_channel::CryptoChannelRes1),
    CryptoChannelReq2(crypto_channel::CryptoChannelReq2),
    CryptoChannelRes2(crypto_channel::CryptoChannelRes2),

    // identificate
    IdentificateReq1(identificate::IdentificateReq1),
    IdentificateRes1(identificate::IdentificateRes1),
    IdentificateReq2(identificate::IdentificateReq2),
    IdentificateRes2(identificate::IdentificateRes2),

    // error
    Error(DataError)
}

impl Message {
    pub fn 
}


impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::CryptoChannelReq1(_) => write!(f, format!("{}", headers::CRYPTO_CHANNEL_REQ1_HEADER)),
            Message::CryptoChannelRes1(_) => write!(f, format!("{}", headers::CRYPTO_CHANNEL_RES1_HEADER)),
            Message::CryptoChannelReq2(_) => write!(f, format!("{}", headers::CRYPTO_CHANNEL_REQ2_HEADER)),
            Message::CryptoChannelRes2(_) => write!(f, format!("{}", headers::CRYPTO_CHANNEL_RES2_HEADER)),

            Message::IdentificateReq1(_) => write!(f, format!("{}", headers::IDENTIFICATE_REQ1_HEADER)),
            Message::IdentificateRes1(_) => write!(f, format!("{}", headers::IDENTIFICATE_RES1_HEADER)),
            Message::IdentificateReq2(_) => write!(f, format!("{}", headers::IDENTIFICATE_REQ2_HEADER)),
            Message::IdentificateRes2(_) => write!(f, format!("{}", headers::IDENTIFICATE_RES2_HEADER)),

            Message::Error(_) => write!(f, format!("{}", headers::ERROR_HEADER)),
        }
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_map(Some(2))?;
        state.serialize_entry("header", &self.to_string());
        match self {
            Message::CryptoChannelReq1(x) => state.serialize_entry("data", &x),
            Message::CryptoChannelRes1(x) => state.serialize_entry("data", &x),
            Message::CryptoChannelReq2(x) => state.serialize_entry("data", &x),
            Message::CryptoChannelRes2(x) => state.serialize_entry("data", &x),

            Message::IdentificateReq1(x) => state.serialize_entry("data", &x),
            Message::IdentificateRes1(x) => state.serialize_entry("data", &x),
            Message::IdentificateReq2(x) => state.serialize_entry("data", &x),
            Message::IdentificateRes2(x) => state.serialize_entry("data", &x),

            Message::Error(x) => state.serialize_entry("data", &x),
        };
        state.end()
    }
}

struct MessageVisitor;
impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a map with keys 'header' and 'data'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>
    {
        let mut header = None;
        let mut data = None;

        while let Some(k) = map.next_key::<&str>()? {
            if k == "header" {
                header = Some(map.next_value::<&str>()?);
            }
            else if k == "data" {
                data = Some(map.next_value::<serde_json::Value>()?);
            }
            else {
                return Err(serde::de::Error::custom(&format!("Invalid key: {}", k)));
            }
        }

        if header.is_none() || data.is_none() {
            return Err(serde::de::Error::custom("Missing header or data"));
        }

        let header = header.unwrap();
        let data = data.unwrap();
        Ok(match header {
            headers::CRYPTO_CHANNEL_REQ1_HEADER => Message::CryptoChannelReq1(value_to_data::<crypto_channel::CryptoChannelReq1, M>(data)?),
            headers::CRYPTO_CHANNEL_RES1_HEADER => Message::CryptoChannelRes1(value_to_data::<crypto_channel::CryptoChannelRes1, M>(data)?),
            headers::CRYPTO_CHANNEL_REQ2_HEADER => Message::CryptoChannelReq2(value_to_data::<crypto_channel::CryptoChannelReq2, M>(data)?),
            headers::CRYPTO_CHANNEL_RES2_HEADER => Message::CryptoChannelRes2(value_to_data::<crypto_channel::CryptoChannelRes2, M>(data)?),

            headers::IDENTIFICATE_REQ1_HEADER => Message::IdentificateReq1(value_to_data::<identificate::IdentificateReq1, M>(data)?),
            headers::IDENTIFICATE_REQ1_HEADER => Message::IdentificateRes1(value_to_data::<identificate::IdentificateRes1, M>(data)?),
            headers::IDENTIFICATE_REQ1_HEADER => Message::IdentificateReq2(value_to_data::<identificate::IdentificateReq2, M>(data)?),
            headers::IDENTIFICATE_REQ1_HEADER => Message::IdentificateRes2(value_to_data::<identificate::IdentificateRes2, M>(data)?),

            headers::ERROR_HEADER => Message::Error(value_to_data::<DataError, M>(data)?),

            _ => {
                return Err(serde::de::Error::custom("認識できないヘッダです"))
            }
        })
    }
}