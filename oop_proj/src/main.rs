extern crate oop_proj;

use oop_proj::oo::demo1;
use oop_proj::oo::demo2;
use oop_proj::oo::demo3;

fn main(){
    let mut a: Option<String> = Some(String::from("dd"));
    println!("{:?}",a.take());

    //demo1::kankan();
    //demo2::kankan();
    demo3::kankan();
}