#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entity { x: f32, y: i32, z: std::string::String, w: Another }

#[derive(Serialize, Deserialize, Debug)]
struct Another { x: std::string::String }

#[derive(Serialize, Deserialize, Debug)]
enum message {
    EntityMsg(Entity),
    AnotherMsg(Another),
}
//
//impl message::Entity {
//    fn test() -> i32{
//        return self::y;
//    }
//}

fn main() {
    let entity = message::EntityMsg(Entity { x: 1.0, y: 2, z: "sss".to_string(), w: Another { x: "aaa".to_string() } });
    let encode = serialize(&entity).unwrap();
    println!("{:?}", encode);
    let decode: message = deserialize(&encode).unwrap();
    match  decode {
        message::EntityMsg(x) => println!("entity: {:?}",x),
        message::AnotherMsg(x) => println!("Another: {:?}",x),
        _ => println!("err"),
    }
    println!("{:?}", decode);
}