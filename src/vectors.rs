

pub fn run(){
    let my_vector: Vec<i32> = Vec::new();

    let numbers: Vec<i32> = vec![1,2,3,4,5];
    //tipo i32, 5 elements

    println!("{:?}", numbers);
    println!("Single value {}", numbers[0]);

    let mut numbers_mut: Vec<i32> = vec![1,2,3,4,5];
    numbers_mut[2] = 20;

    //add value
    numbers_mut.push(5);
    numbers_mut.push(6);

    println!("Single value {:?}", numbers_mut);

    //pop
    numbers_mut.pop();

    println!("Single value {:?}", numbers_mut);

    println!("Single value {}", numbers_mut[2]);

    println!("Single len {}", numbers_mut.len());

    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers_mut));
    //use std::mem para solo usar mem::size_of_va()

    //get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    let slice2: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice2);

    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //mut value
    for x in numbers_mut.iter_mut(){
        *x *= 2;
        println!("Number: {}", x);
    }

    println!("Number mut: {:?}", numbers_mut);
}