pub fn run(){
    let mut x = 10;

    {

        let x = 20;
    }

    let x = "hola mundo";
    println!("x: {}", x);

    let x = true;
    println!("x: {}", x)
}