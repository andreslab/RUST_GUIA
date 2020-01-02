use std::collections::HashMap;

pub fn run() {
    let mut marks = HashMap::new();

    //add values
    marks.insert("Rust programs", 96);
    marks.insert("Web programs", 96);
    marks.insert("UX Design", 96);
    marks.insert("python programs", 96);
    marks.insert("concept programs", 96);

    //find length of hasmaps
    println!("How many subject: {}", marks.len());

    //print value
    match marks.get("Web programs") {
        Some(mark) => println!("you got {}", mark),
        None => println!("not found"),
    }

    //remove
    marks.remove("UX Design");

    //loop
    for (subject, mark) in &marks{
        println!("For {} you got {}", subject, mark);
    }

    //check for value
    println!("dod you study c++ {}", marks.contains_key("C++ programing"));
}