// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// Entity
struct Entity {
    x: u32,
    y: u32,
    integrity: u32,
}

trait Position {
    fn get_position(&self) -> (u32, u32);
    fn set_position(&mut self, x: u32, y: u32);
}

trait Integrity {
    fn get_integrity(&self) -> u32;
    fn set_integrity(&mut self, integrity: u32);
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

impl Integrity for Entity {
    fn get_integrity(&self) -> u32 {
        return self.integrity;
    }
    fn set_integrity(&mut self, integrity: u32) {
        self.integrity = integrity;
    }
}

// Person
struct Person {
    entity: Entity,
    name: String,
    height: u32,
    weight: u32,
    health: u32,
}

trait Profile {
    fn get_profile(&self) -> (Entity, String, u32, u32, u32);
}

impl Profile for Person {
    fn get_profile(&self) -> (Entity, String, u32, u32, u32) {
        return (self.entity, self.name, self.height, self.weight, self.health);
    }
}


fn main() {
    let e1: Entity = Entity{
        x:0, 
        y:0,
        integrity: 100
    };
    println!("{:?}", e1.get_integrity());
    println!("{:?}", e1.get_position());
    let p1: Person = Person {
        entity: Entity { 
            x: 0, 
            y: 0, 
            integrity: 0 
        },
        name: "Origami".to_owned(),
        height: 175,
        weight: 55,
        health: 100
    };
}
