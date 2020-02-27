use std::process::Command;

pub fn run(){
    //python dcode.py
    let mut cmd = Command::new("python3"); //name of programm
    cmd.arg("src/showmsg.py");

    //execute the command
    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("output: {}", String::from_utf8_unchecked(o.stdout) );
            }
            
        },
        Err(_) => println!("Error")
    }
}