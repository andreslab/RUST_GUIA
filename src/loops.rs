pub fn run (){
    let mut n = 0;

    /*loop {
        n += 1;
        println!("the value is {}",n);
    }*/

    /*loop {
        n += 1;

        if n == 7 {
            continue; //skip
        }

        if n > 20 {
            break;
        }
        println!("the value is {}", n);
    }*/

    /*while n <= 10 {
        n += 1;
        println!("the value is {}", n);
    }*/

    let numbers = 1..10;

    for u in 1..11 {
        println!("the number is {}", u);
    }

    for u in numbers {
        println!("the number is {}", u);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for a in animals.iter() {
        println!("the animal is {}", a);
    }

    for (index, a) in animals.iter().enumerate(){
        println!("the animal is {} and its porsition is {}", a, index);
    }

}