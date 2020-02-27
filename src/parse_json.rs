extern crate serde_json;

// extern crate serde;

//#[macro_use] 

extern crate serde_derive;

use serde::{Serialize, Deserialize};


use serde_json::Value as JsonValue;


pub fn run(){
    let json_str = r#"
    
        {
            "name": "jaime",
            "age": 28,
            "is_male": true
        }
    "#;
    let res = serde_json::from_str(json_str);
    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        println!("the name is: {}", p["name"]);
    }else{
        println!("Json structure error");
    }
}


//automatic
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool
}

pub fn json_parse_deserialize(){

    /* let p = Person {name: String::from_str("jaime"), age: 12, is_male:true};

    let serialized = serde_json::to_string(&p).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {}", deserialized);
 */
}