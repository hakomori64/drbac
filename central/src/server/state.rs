use common::db::models::actor::Actor;
use common::pki::Certificate;

#[derive(Clone)]
pub struct State {
    pub opponent_actor: Option<Actor>,
    pub box_secret_key: Option<Vec<u8>>,
    pub box_public_key: Option<Vec<u8>>,
    pub box_certificate: Option<Certificate>
}

impl State {
    /// Create new State
    /// 
    /// stream is tcp stream
    /// 
    /// # Panic
    /// 
    pub fn new(
        opponent_actor: Option<Actor>,
        box_secret_key: Option<Vec<u8>>,
        box_public_key: Option<Vec<u8>>,
        box_certificate: Option<Certificate>,
    ) -> State {
        State {
            opponent_actor,
            box_secret_key,
            box_public_key,
            box_certificate,
        }
    }

    pub fn opponent_actor(&self) -> Option<Actor> {
        self.opponent_actor.clone()
    }

    pub fn box_secret_key(&self) -> Option<Vec<u8>> {
        self.box_secret_key.clone()
    }

    pub fn box_public_key(&self) -> Option<Vec<u8>> {
        self.box_public_key.clone()
    }

    pub fn box_certificate(&self) -> Option<Certificate> {
        self.box_certificate.clone()
    }
}