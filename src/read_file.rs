use std::fs::File;
use std::io::prelude::*;

pub fn run(){
    let mut file = File::open("archivo.txt").expect("Cant open file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Ops! can not read the file");

    println!("File content: \n\n{}", contents);
}