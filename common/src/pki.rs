use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use pem::{Pem, parse, encode};
use std::fs::File;
use std::convert::TryInto;
use std::io::Write;
use std::path::PathBuf;
use rand_core::OsRng;
use ed25519_dalek::{
    Keypair,
    PublicKey,
    SecretKey,
    Signature,
    Signer,
    Verifier,
};
use sha2::{Sha512, Digest};
use base64;
use std::io::BufReader;

pub fn generate_key_pair() -> Result<(Vec<u8>, Vec<u8>)> {

    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let publickey = PublicKey::from_bytes(&keypair.to_bytes()[32..64]).unwrap();
    let secretkey = SecretKey::from_bytes(&keypair.to_bytes()[0..32]).unwrap();

    Ok((secretkey.to_bytes().to_vec(), publickey.to_bytes().to_vec()))
}

pub fn hash(data: &[u8]) -> Result<Vec<u8>> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

pub fn read_pem(path: &PathBuf) -> Result<Vec<u8>> {
    let pem_string = std::fs::read_to_string(path)?;
    let data = parse(&pem_string)?;
    Ok(data.contents)
}

pub fn parse_pem(pem_string: String) -> Result<Vec<u8>> {
    let data = parse(&pem_string)?;
    Ok(data.contents)
}

pub fn create_pem(path: &PathBuf, tag: String, data: Vec<u8>) -> Result<()> {
    if path.exists() {
        return Err(anyhow!("指定されたパスにはすでにファイルが存在します"));
    }

    let parent = path.parent().unwrap();
    if !parent.exists() {
        std::fs::create_dir_all(&parent)?;
    }

    let pem = Pem {
        tag: tag,
        contents: data
    };

    File::create(path)?.write_all(encode(&pem).as_bytes())?;

    Ok(())
}

pub fn sign(message: &[u8], secret_key: &[u8]) -> Result<[u8; 64]> {
    let secret_key = SecretKey::from_bytes(secret_key)?;
    let public_key: PublicKey = (&secret_key).into();
    
    let mut keys = secret_key.as_bytes().clone().to_vec();
    keys.append(&mut public_key.as_bytes().clone().to_vec());
    let key_pair = Keypair::from_bytes(&keys)?;

    Ok(key_pair.sign(message).to_bytes())
}

pub fn verify(signature: [u8; 64], message: &[u8], public_key: &[u8]) -> Result<()> {
    let public_key = PublicKey::from_bytes(public_key)?;
    let signature = Signature::new(signature);
    public_key.verify(message, &signature).map_err(|_| anyhow!("verification failed"))
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq
)]
pub enum BoxType {
    Central,
    Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CSR {
    pub name: String,
    pub box_type: BoxType,
    pub public_key: String // base64 encoded public_key. public_key itself is vec<u8>
}

impl CSR {
    pub fn new(name: String, box_type: BoxType, public_key: Vec<u8>) -> CSR {
        CSR {
            name,
            box_type,
            public_key: base64::encode(public_key)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certificate {
    pub name: String,
    pub box_type: BoxType,
    pub public_key: String,
    pub hash: String // to_string(CSR) | sign with CA secret key
}

impl Certificate {

    pub fn decoded_public_key(&self) -> Result<Vec<u8>> {
        Ok(base64::decode(self.public_key.clone())?)
    }
}

// below functions are not used in production mode 

pub fn generate_csr(name: String, box_type: BoxType, public_key: &[u8]) -> Result<CSR> {
    Ok(CSR::new(name, box_type, public_key.to_vec()))
}

pub fn write_json<T: Serialize>(filename: &PathBuf, data: T) -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    let text = serde_json::to_string(&data)?;
    write!(&file, "{}", text)?;

    Ok(())
}

pub fn read_json<T: DeserializeOwned>(filename: &PathBuf) -> Result<T> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
    
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

pub fn create_certificate(csr: &CSR, ca_secret_key: Vec<u8>) -> Result<Certificate> {
    let csr_text = serde_json::to_string(csr)?;
    let hashed = hash(csr_text.as_bytes())?;
    let signed = sign(&hashed, &ca_secret_key)?;
    let base64ed = base64::encode(&signed);

    Ok(Certificate {
        name: csr.name.clone(),
        box_type: csr.box_type,
        public_key: csr.public_key.clone(),
        hash: base64ed
    })
}

pub fn verify_certificate(certificate: Certificate, ca_public_key: Vec<u8>) -> Result<bool> {
    let csr = CSR {
        name: certificate.name.clone(),
        box_type: certificate.box_type.clone(),
        public_key: certificate.public_key.clone()
    };

    let csr_text = serde_json::to_string(&csr)?;
    let hashed = hash(csr_text.as_bytes())?;
    
    let signed = base64::decode(&certificate.hash)?;
    let signature: [u8; 64] = match signed.try_into() {
        Ok(ba) => ba,
        Err(_) => return Err(anyhow!("signatureの形式が正しくありません"))
    };

    let result = match verify(signature, &hashed, &ca_public_key) {
        Ok(_) => true,
        _ => false
    };

    Ok(result)
}