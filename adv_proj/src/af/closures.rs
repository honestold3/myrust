fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn kankan() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

}

pub fn kankan1(){
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
}

pub fn kankan2(){
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
}

fn returns_closure<T>() -> Box<Fn(T) -> T> {
    Box::new(|x| x )
}

pub fn kankan3(){
    let k = returns_closure();
    let kk = k(String::from("ddd"));
    println!("{:?}",kk);
}