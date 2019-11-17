pub fn run(){
    

    let player_direccion:Direccion = Direccion::Up;

    match player_direccion {
        Direccion::Up => println!("define up"),
        Direccion::Down => println!("define down"),
        Direccion::Right => println!("define right"),
        Direccion::Left => println!("define left")
    }
}

enum Direccion {
    Up,
    Down,
    Right,
    Left
}
