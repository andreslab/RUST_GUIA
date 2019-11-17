pub fn run(){
    let mut x = 10;

    let referencia_x = &x;

    println!("num: {}",referencia_x);

    //------------------------------

    let mut y = 24;

    //perite que se modifique el valor de y mediante referencia_y
    let referencia_y = &mut y;

    *referencia_y += 1;

    println!("num ref mut: {}", referencia_y);

    //-------------------------------

    let mut a = 3;

    {
        let referencia_a = &mut a;

        *referencia_a += 1;

    }

    println!("num: {}",a); //si la reference_a no estuviese dentro un block daria error porque cuando haces referencia no puedes llamar al original
}