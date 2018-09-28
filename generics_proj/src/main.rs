extern crate generics_proj;

use generics_proj::syntax::generic;
use generics_proj::syntax::traits;
use generics_proj::syntax::lifetime;

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn kk(&self) -> u8{
        self.x + 10
    }
}

fn kankan(){
    let p = &Point{
        x: 1,
        y: 2,
    };
    println!("Hello, world  ! {:?}",p.kk());
}


fn main() {
    //kankan()
    //generic::kankan();
    //generic::kankan1();
    //traits::kankan();

    //lifetime::kankan();
    //lifetime::kankan1();

//    let a = String::from("qqq");
//    let b = &a;
//    let c = String::from("gggg");
//
//
//    println!("{}",a);
//    kkkk(a.as_str(),c.as_str());
    lifetime::kankan3();
}

fn kkkk<'a>(a: &'a str, b: &'a str) ->  &'a str {
    a
}
