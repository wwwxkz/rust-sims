#![allow(dead_code)]

use std::fmt::{self, Debug};
use std::{collections::HashMap};

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }


#[derive(Debug, Clone, Copy)]
struct Entity {
    x: u32,
    y: u32,
    integrity: u32,
}

#[derive(Clone)]
struct Person {
    entity: Entity,
    name: String,
    height: u32,
    weight: u32,
    health: u32,
}

#[derive(Clone, Debug)]
enum Order {
    Carnivora,
    Herbivore,
    Omnivore
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
enum Class {
    Fish, 
    Amphibian, 
    Reptile, 
    Mammal, 
    Bird
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
struct Animal {
    entity: Entity,
    name: String,
    height: u32,
    weight: u32,
    health: u32,
    species: String,
    family: String,
    order: Order,
    class: Class
}

trait Position {
    fn get_position(&self) -> (u32, u32);
    fn set_position(&mut self, x: u32, y: u32);
}

trait Integrity {
    fn get_integrity(&self) -> u32;
    fn set_integrity(&mut self, integrity: u32);
}

trait Profile {
    fn get_profile(&self) -> (Entity, HashMap<String, String>);
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


fn main() {
    let e1: Entity = Entity{
        x:0, 
        y:0,
        integrity: 100
    };
    println!("Position: {:?}", e1.get_position());
    println!("Integrity: {:?}", e1.get_integrity());
    let p1: Person = Person {
        entity: Entity { 
            x: 0, 
            y: 0, 
            integrity: 100
        },
        name: "Origami".to_owned(),
        height: 175,
        weight: 55,
        health: 100
    };
    println!("Position: {:?}", p1.get_position());
    println!("Integrity: {:?}", p1.get_integrity());
    println!("Profile: {:?}", p1.get_profile());
    let a1: Animal = Animal {
        entity: Entity {
            x: 0,
            y: 0,
            integrity: 100
        },
        name: "Giant panda".to_string(),
        height: 80,
        weight: 140,
        health: 100,
        species: "Ailuropoda melanoleuca".to_string(),
        family: "Ursidae".to_string(),
        order: Order::Carnivora,
        class: Class::Mammal
    };
    println!("Position: {:?}", a1.get_position());
    println!("Integrity: {:?}", a1.get_integrity());
    println!("Profile: {:?}", a1.get_profile());
}
