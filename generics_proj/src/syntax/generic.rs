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