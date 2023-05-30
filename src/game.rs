use crate::entities::*;

pub struct Game {
    pub entities: Vec<Entities>
}

pub trait Status {
    fn entities_number(&self) -> u32;
}

impl Status for Game {
    fn entities_number(&self) -> u32 {
        return self.entities.len() as u32;
    }
}
