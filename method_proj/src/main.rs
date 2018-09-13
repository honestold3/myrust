#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{ width: size, height: size}
    }
}

impl Rectangle {
    fn area1(self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn area2(&mut self) -> u32 {
        self.height = 40;
        self.height = self.width;
        self.height
    }
}

fn main() {
    //kankan()
    //kankan1()
    //kankan2()
    //kankan0()
    kankan3()
}

fn kankan(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}

fn kankan0(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area1());
    // 由于area1方法中用的是self是值的传递，所有权用完就drop了。在打印就报错了。
    //println!("The area of the rectangle is {} square pixels.", rect1.area1());
}

fn kankan1(){
    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 60, height: 45};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn kankan2(){
    let a= Rectangle::square(30);
    println!("{:?}",a);
    haha(&a);
    println!("again {:?}", a);
}

fn haha(rect: &Rectangle) {
    println!("haha")
}

fn kankan3(){
    let mut rect = Rectangle { height: 30, width: 330 };
    println!("The area of rectangle is {} square pixels. {}",rect.area2(),rect.height);
}
