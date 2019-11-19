//mod print;
//mod vars;
//mod types;
//mod strings;
//mod tuplas;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod enums;
//mod function;
//mod code_block;
//mod shadowing;
//mod reference;
//mod structs;
//mod implementation;
//mod read_file;
//mod command_line;
mod write_file;

const MAXIMUN_NUMBER: u8 = 20;

fn main() {
    println!("Hello, world!");
    //inmutable variable
    //let x = 45;
    
    //mutable variable
    let mut x = 45;
    println!("the value x is {}",x);


    x = 60;
    println!("the value x is {}",x);

    //specify type
    let x: i64 = 45;
    let x: u64 = 45; //no support negative number (unsign)
    let f: f32 = 6.7; //float f32
    let b: bool = false;

    for n in 1..MAXIMUN_NUMBER {
        println!("{}",n);
    }

    //usar un archivo externo
    //print::run();
    //vars::run();
    //types::run();
    //strings::run();
    //tuplas::run();
    //arrays::run();
    //conditionals::run();
    //loops::run();
    //enums::run();
    //function::run();
    //code_block::run();
    //shadowing::run();
    //reference::run();
    //structs::run();
    //implementation::run();
    //read_file::run();
    //command_line::run();
    write_file::run();
}
