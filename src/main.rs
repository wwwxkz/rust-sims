use crate::entities::*;
pub mod entities;

use crate::game::*;
pub mod game;

fn main() {
    let e1: entities::Entity = entities::Entity {
        x: 0,
        y: 0,
        integrity: 100,
    };
    let p1: entities::Person = entities::Person {
        entity: entities::Entity {
            x: 0,
            y: 0,
            integrity: 100,
        },
        name: "Origami".to_owned(),
        height: 175,
        weight: 55,
        health: 100,
    };
    let a1: entities::Animal = entities::Animal {
        entity: entities::Entity {
            x: 0,
            y: 0,
            integrity: 100,
        },
        name: "Giant panda".to_string(),
        height: 80,
        weight: 140,
        health: 100,
        species: "Ailuropoda melanoleuca".to_string(),
        family: "Ursidae".to_string(),
        order: entities::Order::Carnivora,
        class: entities::Class::Mammal,
    };
    let o1: entities::Object = entities::Object {
        entity: entities::Entity {
            x: 0,
            y: 0,
            integrity: 100,
        },
        name: "Refrigerator".to_string(),
        rarity: entities::Rarity::Common,
        exterior: entities::Exterior::Good,
        color: "C2C2C2".to_string(),
        price: 3250,
        category: entities::Category::Kitchen,
    };
    let i1: entities::Item = entities::Item {
        entity: entities::Entity {
            x: 0,
            y: 0,
            integrity: 100,
        },
        name: "Light sword".to_string(),
        rarity: entities::Rarity::Rare,
        exterior: entities::Exterior::Good,
        color: "FFFFFF".to_string(),
        price: 250,
        category: entities::Category::Weapon,
    };
    let game: Game = Game {
        entities: vec![
            Entities::Entity(e1), 
            Entities::Person(p1),
            Entities::Animal(a1),
            Entities::Object(o1),
            Entities::Item(i1),
        ],
    };
    println!("{:?}", game.entities_number());
}
