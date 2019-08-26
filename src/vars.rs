

//variables hold primitive data or references to data
//variables are inmutable by default
//rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 37; //para que sea mutable y se pueda cambiar

    age = 20;

    println!("My name is {} and I am {}", name, age);

    //define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Brad", 24);
    println!("{} is {}", my_name, my_age);

}