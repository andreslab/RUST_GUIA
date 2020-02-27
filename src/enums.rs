#![allow(dead_code)]
//elimina el codigo q no se usa como los enum de Day que no se usan

pub fn run(){
    

    let player_direccion:Direccion = Direccion::Up;

    match player_direccion {
        Direccion::Up => println!("define up"),
        Direccion::Down => println!("define down"),
        Direccion::Right => println!("define right"),
        Direccion::Left => println!("define left")
    }

    let name = String::from("Domenic");

    println!("Character at index 8: {}", match name.chars().nth(1) {
        Some(c) => c.to_string(),
        None => "No character at index 8".to_string()
    });

    println!("Occupation is {}", match get_ocupattion("domenic") {
        Some(o) => o,
        None => "Sorry"
    });
}

//optional enum
fn get_ocupattion(name: &str) -> Option<&str> {
    match name{
        "domenic" => Some("Software developer"),
        "michael" => Some("Dentist"),
        _ => None
    }
}

enum Direccion {
    Up,
    Down,
    Right,
    Left
}


pub enum Day {
    Lunes, 
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,Domingo
}

impl Day {
    pub fn is_weekday(&self) -> bool {
        match self{
            &Day::Sabado | &Day::Domingo => return false,
            _ => return true
        }
    }
}