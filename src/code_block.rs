pub fn run(){
    let x = 10;

    {
        let y = 20;
        println!("x: {} y: {}", x,y);
    }
    //error
    //println!("x: {} y: {}", x,y);
}