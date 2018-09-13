fn main() {
    //kankan()
    //kankan1()
    //kankan2()
    //kankan3()
    kankan4()
}

fn kankan(){
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("word is {}", word)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { //判断是否空格
            return  i;
        }
    }
    s.len()
}

fn kankan1(){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("11:{},{}:22",hello,world);

    let len = s.len();
    let slice = &s[3..len];
    let slice1 = &s[3..];
    println!("slice:{},{}",slice,slice1);

    let ss = &s[0..len];
    let ss1 = &s[..];
    println!("ss::{},{}",ss,ss1)
}

fn kankan2(){
    let mystr = "abc";
    println!("{}",&mystr);
    println!("{}",&mystr[0..1]);

    println!("addr::{:p}", &mystr[0..1]);
    println!("addr::{:p}", &mystr[1..2]);
    println!("addr::{:p}", &mystr[2..3]);
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn kankan3(){
    let mut s = String::from("hello world");
    {
        let word = first_word1(&s);
        println!("word::{}",word);
    }

    //回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。
    // 因为 clear 需要清空 String ，它尝试获取一个可变引用，它失败了。
    // Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！
    s.clear(); //error
}

fn kankan4(){
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("{}",slice[1]);
}
