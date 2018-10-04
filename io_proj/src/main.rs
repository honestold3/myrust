use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
//    let args : Vec<String> = env::args().collect();
//    println!("{:?}",args);
//
//    let query = &args[1];
//    let filename = &args[2];
//    println!("Searching for {}", query);
//    println!("In file {}", filename);
//
//    let mut f = File::open(filename).expect("file not found");
//    let mut contents = String::new();
//    f.read_to_string(&mut contents).expect("something went wrong reading the file");
//
//    println!("With text:\n{}", contents);

    //------------------------------------------------------------
    let args: Vec<String> = env::args().collect();
    //let (query, filename) = parse_config(&args);
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    let mut f = File::open(config.filename).expect("file not found");

    //---------------------------------------------------------------

    let config = Config::new(&args);

//    let config = Config::new(&args).unwrap_or_else(|err| {
//
//        println!("Problem parsing arguments: {}", err);
//
//        process::exit(1);
//
//    });

    let config = Config::new1(&args).unwrap_or_else(|err| {

        println!("Problem parsing arguments: {}", err);

        process::exit(1);

    });

    println!("{:?}",config)
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}

//fn parse_config(args: &[String]) -> (&str, &str) {
//    let query = &args[1];
//    let filename = &args[2];
//    (query, filename)
//}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }

}

impl Config {
    fn new1(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })

    }

}
