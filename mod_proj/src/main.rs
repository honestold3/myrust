extern crate my_lib;

use my_lib::client::connect;
use my_lib::networks::hehe;
use my_lib::networks::kankan;

fn main() {
    //kankan();
    kankan1();
//    let s = String::from("hello");
//    let s1 = s;
//    println!("{}",s);
}

fn kankan(){
    my_lib::client::connect();
    println!("Hello, world!");
    hehe::kankan1();
    kankan::kankan();
}

//借引：不能在拥有不可变引用的同时拥有可变引用
fn kankan1(){
    let mut a = String::from("hello");

    let b = &a;

    //let  c = &mut a; //error

    //println!("{:?}",b)


}
