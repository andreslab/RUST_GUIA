pub fn run(){
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);


    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let coleccion = (12,23,456);
    println!("{}", coleccion.2);

    let coleccion2 = ("hola", 12, 3.41);
    println!("{}", coleccion2.1);

    let coleccion3 = ("hola", (1,2,4,5));
    println!("{}", (coleccion3.1).2);

    let coleccion4 = (234,34,12);
    let (a, b, c) = coleccion4;
    print!("{}",a );
}