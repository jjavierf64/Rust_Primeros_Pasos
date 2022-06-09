use std::env;

pub fn run(){

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", &args);
    
    let command = args[1].clone();
    let name = args[2].clone();
    let status = "100%";

    println!("Command: {}", command);



    if command=="salute"{
        println!("Hello back, {}", name);
    } else if command == "status"{
        
        println!("Status: {}", status);
    }

}