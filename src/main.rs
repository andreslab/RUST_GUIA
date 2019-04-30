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
}
