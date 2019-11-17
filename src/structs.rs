pub fn run(){
    let mut color_bg = Color{ red: 255, blue: 34, green: 10};

    color_bg.blue = 43;

    println!("{}, {}, {}", color_bg.red, color_bg.blue, color_bg.green);


    let mut color_s = ColorSimple(200,43,12);
    println!("{}, {}, {}", color_s.0, color_s.1, color_s.2);

    let blue = Color {red: 0, blue: 255, green: 0};
    print_color(&blue); //se pasa la referencia para que se pueda llamar cuantas veces quiere
    print_color(&blue); 
    print_color(&blue); 
    print_color(&blue); 
}

struct Color {
    red: u8, //0-255
    blue: u8,
    green:u8
}

struct ColorSimple(u8,u8,u8);

fn print_color(c: &Color){
    println!("color: {}, {}, {}", c.red, c.blue, c.green);
}