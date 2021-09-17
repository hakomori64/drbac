use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoChannelReq1 {
    pub public_key: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoChannelRes1 {
    pub public_key: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoChannelReq2 {
    pub ping: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoChannelRes2 {
    pub ping: String,
}
