use common::state::StateTrait;
use common::pki::{
    Certificate
};

#[derive(Clone)]
pub struct State {
    box_secret_key: Option<Vec<u8>>,
    box_public_key: Option<Vec<u8>>,
    box_certificate: Option<Certificate>,
}

impl StateTrait for State {
    
    fn box_secret_key(&self) -> Option<Vec<u8>> {
        self.box_secret_key.clone()
    }
    
    fn box_public_key(&self) -> Option<Vec<u8>> {
        self.box_public_key.clone()
    }
    
    fn box_certificate(&self) -> Option<Certificate> {
        self.box_certificate.clone()
    }
}

impl State {
    pub fn new(
        box_secret_key: Option<Vec<u8>>,
        box_public_key: Option<Vec<u8>>,
        box_certificate: Option<Certificate>,
    ) -> State {
        State {
            box_secret_key,
            box_public_key,
            box_certificate,
        }
    }
}