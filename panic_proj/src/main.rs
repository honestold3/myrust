extern crate panic_proj;

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //kankan_backtrace();
    //kankan1();
    //kankan2();
    //kankan3();
    //kankan4();
    //kankan5();

    guess();
}

fn kankan(){
    panic!("crash and burn");
}

fn kankan_backtrace(){
    let v = vec![1,2,3];
    v[99];
}

fn kankan1(){
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) =>{
            panic!("There was a problem opening the file: {:?}",error);
        }
    };
}

fn kankan2(){
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => {
                    println!("create file");
                    fc
                },
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}",e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}",error)
        },
    };
}

fn kankan3(){
    //let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("{:?}", f)
}

fn kankan4(){
    let f = read_username_from_file2();
    println!("{:?}",f)
}

fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
}

fn kankan5(){
    use std::net::IpAddr;

    let home: IpAddr = "127.333.0.1".parse().unwrap();
}

fn guess(){
    use panic_proj::games::guess;

    guess::guess_num();
    //guess::guess_num1();
}
