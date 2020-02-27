extern crate regex;

use regex::Regex;

pub fn run(){
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "dcode";

    println!("Found match? {}", re.is_match(text));
}