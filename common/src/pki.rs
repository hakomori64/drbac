use anyhow::{Result, anyhow};
use pem::{Pem, parse, encode};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use rand_core::OsRng;
use ed25519_dalek::{
    Keypair,
    PublicKey,
    SecretKey
};
use sha2::{Sha512, Digest};

pub const SECRET_FILE_NAME: &str = "secret.pem";
pub const PUBLIC_FILE_NAME: &str = "public.pem";

pub fn generate_key_pair() -> Result<(Vec<u8>, Vec<u8>)> {

    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let publickey = PublicKey::from_bytes(&keypair.to_bytes()[32..64]).unwrap();
    let secretkey = SecretKey::from_bytes(&keypair.to_bytes()[0..32]).unwrap();

    Ok((secretkey.to_bytes().to_vec(), publickey.to_bytes().to_vec()))
}

pub fn hash(data: Vec<u8>) -> Result<Vec<u8>> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

pub fn read_pem(path: &PathBuf) -> Result<Vec<u8>> {
    let pem_string = std::fs::read_to_string(path)?;
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