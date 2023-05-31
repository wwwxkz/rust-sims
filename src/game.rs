use crate::entities::{*, self};

#[derive(Clone)]
pub enum State {
    New,
    Playing,
    Stopped,
    Idle
}

#[derive(Clone)]
pub struct Game {
    pub entities: Vec<Entities>,
    pub time: u32,
    pub day: u32,
    pub state: State
}

pub trait Status {
    fn get_entities(&self) -> Vec<Entities>;
    fn get_entities_number(&self) -> u32;
    fn get_time(&self) -> u32;
    fn set_time(&mut self, time: u32);
    fn get_day(&self) -> u32;
    fn set_day(&mut self, day: u32);
    fn play(&self);
    fn resume(&mut self);
    fn pause(&mut self);
}

impl Status for Game {
    fn get_entities(&self) -> Vec<Entities> {
        return self.entities.clone();
    }
    fn get_entities_number(&self) -> u32 {
        return self.entities.len() as u32;
    }
    fn get_time(&self) -> u32 {
        return self.time;
    }
    fn set_time(&mut self, time: u32) {
        self.time = time;
    }
    fn get_day(&self) -> u32 {
        return self.day;
    }
    fn set_day(&mut self, day: u32) {
        self.day = day;
    }
    fn play(&self) {
        entities::Person {
            entity: entities::Entity {
                x: 0,
                y: 0,
                integrity: 100,
            },
            name: "".to_string(),
            height: 0,
            weight: 0,
            health: 0,
        };
    }
    fn resume(&mut self) {
        self.state = State::Playing;
    }
    fn pause(&mut self) {
        self.state = State::Stopped;
    }
}
