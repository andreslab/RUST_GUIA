use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();

    for argument in args.iter(){
        println!("{}", argument);
    }
    //println!("{}",arg[0]);
}

//al ejecutar
//cargo run argumento1 argumento2