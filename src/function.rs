pub fn run (){
    if is_active(20) {
        println!("numero par");
    }
}

fn is_active(num: u32) -> bool {
    return num % 2 == 0;
}