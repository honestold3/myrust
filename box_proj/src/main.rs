use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;//定义了用于此 trait 的关联类型

    fn deref(&self) ->&Vec<u8> {
        &self.audio
    }
}

fn kankan(){
    let my_favorite_song = Mp3 {
        audio: vec![1,2,3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells Like Teen Spirit")),
    };

    assert_eq!(vec![1,2,3],*my_favorite_song);
}
//----------------------------------------------------------
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}",name);
}

fn kankan1(){
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
//---------------------------------------------------------
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn kankan2() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    drop(c);
    println!("CustomSmartPointers created.");
}

fn main() {
    //kankan();
    //kankan1();
    kankan2();
}


