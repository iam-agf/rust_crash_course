use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name: String = String::from("Brad");
    let status = "100%";

    println!("Args {}", command);

    if command == "hello"{
        println!("Hello {}, how are you", name);
    } else if command == "status"{
        println!("Status {}", status);
    } else{
        println!("Not valid command");
    }
}