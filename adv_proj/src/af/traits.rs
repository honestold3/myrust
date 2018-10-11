pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug)]
struct Counter{}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(6)
    }
}

pub fn kankan(){
    let mut kk = Counter{};
    println!("{:?}",kk.next());
}
//-----------------------------------------------------
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn kankan1() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}

//--------------------------------------------------------
//trait Add<RHS=Self> {
//    type Output;
//    fn add(self, rhs: RHS) -> Self::Output;
//}


#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0 )
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn kankan2(){
    let m =  Millimeters(15);
    let m1 = Millimeters(15);

    let m2 = Meters(15);
    let m3 = Millimeters(15);
    let m11 = m3 + m2;

    println!("m+m1:::{:?}",m+m1);
    println!("m1+m2::{:?}",m11);
}

//---------------------------------------------

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }

}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn kankan3(){
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

//------------------------------------------
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn kankan4() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("kankan {}", <Dog as Animal>::baby_name());
}

//-------------------------------------------------------

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point1 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//----------------------------------------------------------

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn kankan5(){
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

