use anyhow::Result;
use common::pki::{
    generate_key_pair,
    create_pem,
};
use std::path::PathBuf;

fn main() -> Result<()> {

    let secret_path: PathBuf = ["secret_key.pem"].iter().collect();
    let public_path: PathBuf = ["public_key.pem"].iter().collect();
    let (secret_key, public_key) = generate_key_pair()?;
    println!("creating server key at path");
    create_pem(&secret_path, String::from("secret key"), secret_key)?;
    create_pem(&public_path, String::from("public key"), public_key)?;
    Ok(())
}