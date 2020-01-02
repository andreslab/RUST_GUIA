use std::io;

pub fn run (){
    let mut input = String::new();

    println!("hey mate! Say something:");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Success you said: {}", input.to_uppercase());
        },
        Err(e) = {
            println!("ops, something went wrong: {}", e);
        }
    }
}