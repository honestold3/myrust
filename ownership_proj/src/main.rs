fn main() {
    //println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1); //error : use of moved value

    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    println!("-----------------------");

    let s4 = String::from("hello");

    takes_ownership(s4);

    //println!("{}",s4); // error, s4 freed

    let x = 5;

    makes_copy(x);

    println!("{}",x); // not error, x is copy

    println!("-----------------------");

    let s5 = gives_ownership();

    let s6 = String::from("hello");

    let s7 = takes_and_gives_back(s6);
    println!("s7::{}",s7);

    println!("s5::{}",s5); //no error s5 is return value

    //println!("{}", s6); //error s6 is moved

//    let s6 = takes_and_gives_back(s6);
//    println!("s6::{}",s6); //no error s6 is retrun value

    println!("-----------------------");
    let ss1 = String::from("hehe");
    let (ss2, len) = calculate_length(ss1);
    println!("The length of '{}' is {}.", ss2, len);
    //println!("ss1: {}", ss1); // error ss1 is moved

    println!("-----------------------");

    let mut a = Foo { f: Box::new(10) };
    // mutable borrow
    {
        let x = &mut a; //相同生命周期中，只能有一个可变引用
        //println!("{}",x.f);
    }
    //let y = &a;
    // error: cannot borrow `a.f` as immutable because `a` is also borrowed as mutable
    println!("{}", a.f);

    println!("-----------------------");
    let x = String::from("X");
//    let z;
//    {
//        let y = String::from("Y");
//        z = foo(&x, &y);
//    } //
//    println!("z = {}", z);

    println!("-----------------------");
    fin2();

    println!("-----------------------");


}

//它只有一个主人。当然你可以把书“给”其它人，所有权就归其它人
/**
fn fin(){
    let a = String::from("book"); // "book" 归 a 所有
    let b = a; // a 将 "book" 转让给 b
    println!("a = {}", a); // 出错，a 已经无权使用资源
}
**/

//允许租借。你可以先把书“给”别人，别人用完后再“给”你。“借”，则保证对方不会不把书还你。
/**
fn fin1(){
    let a = String::from("book");
    {
        let b = a; // a 将 "book" 转让给 b
        a =a ;
    } // b 死了，却没有将 "book" 还给 a
    println!("a = '{}'", a); // 出错，"book" 不在 a 手上。
}
**/
fn fin2(){
    let mut a = String::from("book");
    let b = &a; // "book" 借给 b 只读
    let c = &a; // "book" 同时 借给 c 只读
    println!("a = '{}'", a);
    println!("b = '{}'", b);
    println!("b = '{}'", c);
}

/**fn foo(x: &str, y: &str) -> &str {
    if random() % 2 == 0 { x } else { y }
}
**/

struct Foo {
    f: Box<u32>,
}

fn owner(){
    let mut a = Foo{f: Box::new(10)};
    let x = &mut a;

    let y = x;

    //println!("{}", x.f);
}


fn takes_ownership(some_string: String) { // some_string comes into scope.

    println!("{}", some_string);

}// Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.

    println!("{}", some_integer);

}// Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string

} // takes_and_gives_back will take a String and return one.

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope.

    a_string

}// a_string is returned and moves out to the calling function.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.
    (s, length)
}