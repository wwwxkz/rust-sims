use std::fmt::{self, Debug};
use std::{collections::HashMap};

pub enum Entities {
    Entity(Entity),
    Person(Person),
    Animal(Animal),
    Object(Object),
    Item(Item)
}

#[derive(Clone, Debug)]
pub enum Rarity {
    Common,
    Rare,
}

#[derive(Clone, Debug)]
pub enum Exterior {
    Good
}

#[derive(Clone, Debug)]
pub enum Category {
    Weapon,
    Kitchen
}

#[derive(Clone, Debug)]
pub enum Order {
    Carnivora,
    Herbivore,
    Omnivore
}

#[derive(Clone, Debug)]
pub enum Class {
    Fish, 
    Amphibian, 
    Reptile, 
    Mammal, 
    Bird
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub integrity: u32,
}

pub struct Object {
    pub entity: Entity,
    pub name: String,
    pub rarity: Rarity,
    pub exterior: Exterior,
    pub color: String,
    pub price: u32,
    pub category: Category
}

pub struct Item {
    pub entity: Entity,
    pub name: String,
    pub rarity: Rarity,
    pub exterior: Exterior,
    pub color: String,
    pub price: u32,
    pub category: Category
}

#[derive(Clone)]
pub struct Person {
    pub entity: Entity,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub health: u32,
}

#[derive(Clone)]
pub struct Animal {
    pub entity: Entity,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub health: u32,
    pub species: String,
    pub family: String,
    pub order: Order,
    pub class: Class
}

pub trait Position {
    fn get_position(&self) -> (u32, u32);
    fn set_position(&mut self, x: u32, y: u32);
}

pub trait Integrity {
    fn get_integrity(&self) -> u32;
    fn set_integrity(&mut self, integrity: u32);
}

pub trait Profile {
    fn get_profile(&self) -> (Entity, HashMap<String, String>);
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Exterior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Position for Entity {
    fn get_position(&self) -> (u32, u32) {
        return (
            self.x,
            self.y
        );
    }
    fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

impl Position for Object {
    fn get_position(&self) -> (u32, u32) {
        return (
            self.entity.x,
            self.entity.y
        );
    }
    fn set_position(&mut self, x: u32, y: u32) {
        self.entity.x = x;
        self.entity.y = y;
    }
}

impl Position for Item {
    fn get_position(&self) -> (u32, u32) {
        return (
            self.entity.x,
            self.entity.y
        )
    }
    fn set_position(&mut self, x: u32, y: u32) {
        self.entity.x = x;
        self.entity.y = y;
    }
}

impl Position for Person {
    fn get_position(&self) -> (u32, u32) {
        return (
            self.entity.x,
            self.entity.y
        );
    }
    fn set_position(&mut self, x: u32, y: u32) {
        self.entity.x = x;
        self.entity.y = y;
    }
}

impl Position for Animal {
    fn get_position(&self) -> (u32, u32) {
        return (
            self.entity.x,
            self.entity.y
        )        
    }
    fn set_position(&mut self, x: u32, y: u32) {
        self.entity.x = x;
        self.entity.y = y;
    }
}

impl Integrity for Entity {
    fn get_integrity(&self) -> u32 {
        return self.integrity;
    }
    fn set_integrity(&mut self, integrity: u32) {
        self.integrity = integrity;
    }
}

impl Integrity for Object {
    fn get_integrity(&self) -> u32 {
        return self.entity.integrity;
    }
    fn set_integrity(&mut self, integrity: u32) {
        self.entity.integrity = integrity;
    }
}

impl Integrity for Item {
    fn get_integrity(&self) -> u32 {
        return self.entity.integrity;
    }
    fn set_integrity(&mut self, integrity: u32) {
        self.entity.integrity = integrity;
    }
}

impl Integrity for Person {
    fn get_integrity(&self) -> u32 {
        return self.entity.integrity;
    }
    fn set_integrity(&mut self, integrity: u32) {
        self.entity.integrity = integrity;
    }
}

impl Integrity for Animal {
    fn get_integrity(&self) -> u32 {
        return self.entity.integrity;
    }
    fn set_integrity(&mut self, integrity: u32) {
        self.entity.integrity = integrity;
    }
}

impl Profile for Object {
    fn get_profile(&self) -> (Entity, HashMap<String, String>) {
        let mut profile: HashMap<String, String> = HashMap::new();
        profile.insert(
            "name".to_string(),
            self.name.clone()
        );
        profile.insert(
            "rarity".to_string(),
            self.rarity.to_string()
        );
        profile.insert(
            "exterior".to_string(),
            self.exterior.to_string()
        );
        profile.insert(
            "color".to_string(),
            self.color.clone()
        );
        profile.insert(
            "price".to_string(),
            self.price.to_string()
        );
        profile.insert(
            "category".to_string(),
            self.category.to_string()
        );
        return (self.entity, profile);
    }
}

impl Profile for Item {
    fn get_profile(&self) -> (Entity, HashMap<String, String>) {
        let mut profile: HashMap<String, String> = HashMap::new();
        profile.insert(
            "name".to_string(),
            self.name.clone()
        );
        profile.insert(
            "rarity".to_string(),
            self.rarity.to_string()
        );
        profile.insert(
            "exterior".to_string(),
            self.exterior.to_string()
        );
        profile.insert(
            "color".to_string(),
            self.color.clone()
        );
        profile.insert(
            "price".to_string(),
            self.price.to_string()
        );
        profile.insert(
            "category".to_string(),
            self.category.to_string()
        );
        return (self.entity, profile);
    }
}

impl Profile for Person {
    fn get_profile(&self) -> (Entity, HashMap<String, String>) {
        let mut profile: HashMap<String, String> = HashMap::new();
        profile.insert(
            "name".to_string(),
            self.name.clone()
        );
        profile.insert(
            "height".to_string(),
            self.height.to_string()
        );
        profile.insert(
            "weight".to_string(),
            self.weight.to_string()
        );
        profile.insert(
            "health".to_string(),
            self.health.to_string()
        );
        return (self.entity, profile);
    }
}

impl Profile for Animal {
    fn get_profile(&self) -> (Entity, HashMap<String, String>) {
        let mut profile: HashMap<String, String> = HashMap::new();
        profile.insert(
            "name".to_string(),
            self.name.clone()
        );
        profile.insert(
            "height".to_string(),
            self.height.to_string()
        );
        profile.insert(
            "weight".to_string(),
            self.weight.to_string()
        );
        profile.insert(
            "health".to_string(),
            self.health.to_string()
        );
        profile.insert(
            "species".to_string(),
            self.species.clone()
        );
        profile.insert(
            "family".to_string(),
            self.family.clone()
        );
        profile.insert(
            "Order".to_string(),
            self.order.to_string()
        );
        profile.insert(
            "class".to_string(),
            self.class.to_string()
        );
        return (self.entity, profile);    
    }
}