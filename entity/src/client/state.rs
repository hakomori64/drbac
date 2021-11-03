use common::db::models::actor::Actor;
use common::pki::{
    BoxType,
    Certificate,
};
use common::state::StateTrait;


#[derive(Clone)]
pub struct State {
    actor: Option<Actor>,
    box_secret_key: Option<Vec<u8>>,
    box_public_key: Option<Vec<u8>>,
    box_certificate: Option<Certificate>,
    opponent_type: Option<BoxType>,
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
    /// Create new State
    /// 
    /// stream is tcp stream
    /// 
    /// # Panic
    /// 
    pub fn new(
        actor: Option<Actor>,
        box_secret_key: Option<Vec<u8>>,
        box_public_key: Option<Vec<u8>>,
        box_certificate: Option<Certificate>,
        opponent_type: Option<BoxType>,
    ) -> State {
        State {
            actor,
            box_secret_key,
            box_public_key,
            box_certificate,
            opponent_type,
        }
    }
    
    pub fn actor(&self) -> Option<Actor> {
        self.actor.clone()
    }
    
    pub fn opponent_type(&self) -> Option<BoxType> {
        self.opponent_type.clone()
    }
    
}