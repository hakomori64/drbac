pub mod handlers;
pub mod common_crypto;

use serde::{Serialize, Deserialize};

use handlers::{
    crypto_channel,
    identificate
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataError {
    pub reason: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub header: String,
    pub data: serde_json::Value
}

enum Data {
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
}

impl Data {
    pub fn get_header(&self) -> String {
        String::from(match self {
            Data::CryptoChannelReq1(_) => "CRYTPO_CHANNEL_REQ1",
            Data::CryptoChannelRes1(_) => "CRYTPO_CHANNEL_RES1",
            Data::CryptoChannelReq2(_) => "CRYTPO_CHANNEL_REQ2",
            Data::CryptoChannelRes2(_) => "CRYTPO_CHANNEL_RES2",

            Data::IdentificateReq1(_) => "IDENTIFICATE_REQ1",
            Data::IdentificateRes1(_) => "IDENTIFICATE_RES1",
            Data::IdentificateReq2(_) => "IDENTIFICATE_REQ2",
            Data::IdentificateRes2(_) => "IDENTIFICATE_RES2",
        })
    }
}