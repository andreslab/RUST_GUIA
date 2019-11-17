pub fn run(){
    let numbers: [i32; 5] = [1,2,3,4,5];
    //tipo i32, 5 elements

    println!("{:?}", numbers);
    println!("Single value {}", numbers[0]);

    let mut numbers_mut: [i32; 5] = [1,2,3,4,5];
    numbers_mut[2] = 20;

    println!("Single value {}", numbers_mut[2]);

    println!("Single len {}", numbers_mut.len());

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers_mut));
    //use std::mem para solo usar mem::size_of_va()

    //get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    let slice2: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice2);

    let array1 = [2; 400]; //400 elementos de valor 2
    for i in 0..array1.len() {
        println!("valor array1: {}", array1[i]);
    }

    let array2: [u8, 4] = [2,5,4,2]; //el tipo no es necesario poner

}