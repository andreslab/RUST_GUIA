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
//mod write_file;
//mod traits;
//mod matchs;
//mod read_file;
//mod hash_maps;
//mod random_number;
//mod regular_expression;
//mod command;
mod parse_json;

/* mod dcode {
    pub fn print_message(){
        println!("How it going");
    }
} */

//mod request_http;

//const MAXIMUN_NUMBER: u8 = 20;

fn main() {
    /*println!("Hello, world!");
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
    }*/

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
    //traits::run();
    //matchs::run();
    //read_file::run();
    //hash_maps::run();
    //random_number::run();
    //regular_expression::run();
    //dcode::print_message();
    //request_http::largeRequest();
    //request_http::shortRequest();

    //let d = enums::Day::Martes;
    //println!("is d a weekday? {}", d.is_weekday());

    //command::run();

    parse_json::run();
}

fn get_two() -> i32 {
    2
}

#[cfg(test)]
mod dcode_test {
    #[test] //assert true es el sucess
    #[should_panic] //panic es el success
    fn test_basic(){
        assert!(1 == 1); //ok
        panic!("Oh no");
    }

    #[test]
    fn test_equals(){
        assert_eq!(2, 1 + 1);
    }

    #[test]
    #[ignore]
    fn test_equals_2(){
        assert_eq!(2, 3 + 1);
    }

    #[test]
    fn test_equa_3(){
        assert_eq!(2, super::get_two()); //con super obtengo los metodos y parametros de fuera
    }
}