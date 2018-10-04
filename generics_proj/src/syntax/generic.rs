use std::cmp::PartialOrd;
use std::fmt::Debug;

//pub fn largest<T: PartialOrd + Copy + Debug>(list: &[T]) -> T {
pub fn largest<T>(list: &[T]) -> T
   where T: PartialOrd + Copy + Debug
{
    let mut largest = list[0];
    for &item in list.iter() {
        println!("{:?}",item);
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn kankan() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

//-------------------------------------------------
pub fn largest1<T: PartialOrd>(list: &[T]) -> &T {
    let mut counter = 0;
    let mut number = 0;
    for item in list.iter() {
        println!("counter---{}", counter);
        if item > &list[number] {
            println!("n: {} --- c: {} ---",number,counter);
            number = counter;
        }
        counter = counter+1;
    }
    return &list[number];
}

pub fn kankan1(){
    let number = vec![34, 50, 25, 100, 65,200];
    let result = largest1(&number);
    println!("The largest number is {}", result);
    let chars = vec!['y', 'm', 'a', 'z'];
    let result = largest1(&chars);
    println!("The largest char is {}", result);
}

//-----------------------------------------------------
#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point1<T,U>{
    x: T,
    y: U,
}

pub fn kankan2(){
    let mix = Point{x: 5.0,y: 2.0};
    println!("{:?}",mix);

    let a: f32 = 0 as f32;
    let b: u8 = 11.5 as u8;

    println!("{}",b)
}

pub fn kankan3(){
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point1 { x: 5, y: 4.0 };
}

//---------------------------------------------------
#[derive(Debug)]
struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn kankan4(){
    let p = Point3{x: 5, y: 10};
    println!("p.x = {}",p.x());
}

//-------------------------------------------------------
struct Point4<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point4<T, U> {
    fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
        Point4 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn kankan5() {
    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}