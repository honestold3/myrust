extern crate ionew_proj;

use std::env;

use std::process;

use ionew_proj::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = ionew_proj::run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }

    kankan();
}

fn kankan(){
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}



