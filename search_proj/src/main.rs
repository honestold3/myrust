extern crate search_proj;

use std::env;

use std::process;

use search_proj::Config;

fn main() {
//    let mut stderr = std::io::stderr();
//    let config = Config::new(env::args()).unwrap_or_else(|err| {
//        writeln!(
//            &mut stderr,
//            "Problem parsing arguments: {}",
//            err
//        ).expect("Could not write to stderr");
//        process::exit(1);
//
//    });
//
//    if let Err(e) = search_proj::run(config) {
//        writeln!(
//            &mut stderr,
//            "Application error: {}",
//            e
//        ).expect("Could not write to stderr");
//        process::exit(1);
//    }

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);

    });
    println!("search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = search_proj::run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }
}
