use std::path::PathBuf;
use anyhow::{Result, anyhow};
use common::io::{
    write_json_to_file,
    read_json_from_file,
};
use common::pki::{
    CSR,
    BoxType,
    generate_key_pair,
    create_pem,
    read_pem,
    generate_csr,
    create_certificate,
};

pub fn generate_keypair_and_csr(name: String, server_type: String) -> Result<()> {
    let (secret_key, public_key) = generate_key_pair()?;
    let secret_name = format!("{}_secret.pem", name);
    let public_name = format!("{}_public.pem", name);
    let csr_name = format!("{}.csr", name);
    let secret_path: PathBuf = [secret_name].iter().collect();
    let public_path: PathBuf = [public_name].iter().collect();
    let csr_path: PathBuf = [csr_name].iter().collect();

    create_pem(&secret_path, String::from(format!("{} secret key", name)), secret_key)?;
    create_pem(&public_path, String::from(format!("{} public key", name)), public_key.clone())?;

    let server_type = match server_type.as_str() {
        "central" => BoxType::Central,
        "client" => BoxType::Client,
        _ => {
            return Err(anyhow!("unrecognized server_type"));
        }
    };

    let csr = generate_csr(name, server_type, &public_key)?;
    write_json_to_file(&csr_path, csr)?;

    Ok(())
}

pub fn sign(csr_path: String) -> Result<()> {
    let ca_secret_path: PathBuf = ["ca_secret.pem"].iter().collect();
    let ca_public_path: PathBuf = ["ca_public.pem"].iter().collect();

    let csr_path: PathBuf = [csr_path].iter().collect();

    if ! ca_secret_path.exists() || ! ca_public_path.exists() || ! csr_path.exists() {
        println!("CAのキーがありません。");
        return Err(anyhow!("CAのキーがありません"));
    }

    let secret_key = read_pem(&ca_secret_path)?;
    let csr = read_json_from_file::<CSR>(&csr_path)?;

    let certificate = create_certificate(&csr, secret_key)?;

    let dest_name = csr.name;
    let dest_name = format!("{}.cert", dest_name);
    let dest_file_path: PathBuf = [dest_name].iter().collect();
    write_json_to_file(&dest_file_path, certificate)?;

    Ok(())
}

pub fn save_ca_key() -> Result<()> {
    let (secret_key, public_key) = generate_key_pair()?;
    let ca_secret_path: PathBuf = ["ca_secret.pem"].iter().collect();
    let ca_public_path: PathBuf = ["ca_public.pem"].iter().collect();
    create_pem(&ca_secret_path, String::from("ca secret key"), secret_key)?;
    create_pem(&ca_public_path, String::from("ca public key"), public_key)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
