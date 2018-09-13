extern crate my_lib;

use my_lib::client::connect;

fn main() {
    my_lib::client::connect();
    println!("Hello, world!");
}
