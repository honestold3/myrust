pub fn kankan(){
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}",x+y);

    let y = Box::new(x);
    println!("{}",*y);

    let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
    f();

    type Thunk = Box<Fn() + Send + 'static>;
    let f1: Thunk = Box::new(|| println!("hi1"));
    f1();
}

#[macro_export]
macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
}

pub fn kankan1(){
    let kk = multiply_add!(1,2,3);
    println!("{}",kk);
}
