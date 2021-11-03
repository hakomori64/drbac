use crate::pki::Certificate;

pub trait StateTrait {
    fn box_secret_key(&self) -> Option<Vec<u8>>;
    fn box_public_key(&self) -> Option<Vec<u8>>;
    fn box_certificate(&self) -> Option<Certificate>;
}