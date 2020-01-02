pub fn run (){
    let number = 2;

    match number {
        1 => println!("it is one"),
        2 => println!("it is two"),
        10 | 11 => println!("it is 10 or 11"),
        12..=20 => println!("it is in rnge bethwen 12 and 20"),
        _ => println!("it doesnt match"),
    }

    let name = "dominic";

    match name {
        "jaime" => println!("It is Jaime"),
        "dominic" => println!("It is dominic"),
        _ => println!("It null"),
    }
}