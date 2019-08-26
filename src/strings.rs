pub fn run(){
    let mut hello = String::from("Hello");

    //get length
    println!("length: {}", hello.len());

    //add char
    hello.push('w');

    //add string
    hello.push_str("orld!");

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    //contain
    println!("contains 'world' {}", hello.contains("world"));

    //replace
    println!("replace: {}", hello.replace("world", " here"));
    
    //loop throuh string by whitespace
    for world in hello.split_whitespace(){
        println!("{}",world );
    }   

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //assertion testing
    assert_eq!(3, s.len());

    println!("{}", hello);
}