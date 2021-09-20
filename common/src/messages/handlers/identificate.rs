use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentificateReq1 {
    pub name: String,
    pub actor_type: String,
    pub public_key_blob: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentificateRes1 { }

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentificateReq2 {
    pub name: String,
    pub actor_type: String,
    pub signature: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentificateRes2 {
    pub common_key: [u8; 32]
}
