fn main() {
    //kankan()
    //kankan1()
    //kankan2()
    //kankan3()
    kankan4()
}

struct  User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn kankan(){
    let user = User{
        email: String::from("kkk@kll.com"),
        username: String::from("hehe"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {

        email: String::from("someone@example.com"),

        username: String::from("someusername123"),

        active: true,

        sign_in_count: 1,

    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("rr@kkk.com"),
        username: String::from("hahah"),
        ..user// User实例设置新的email和username值,其余值来自 user 变量中实例的字段
    };

}

fn build_user(email: String, username: String) -> User {
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
//-------------------------------------------------------------
fn kankan1(){
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.",
    area(width,height));
}

fn area(width: u32,height: u32) -> u32 {
    width * height
}

//-------------------------------------------------------------
fn kankan2(){
    let rect1 = (30,50);
    println!("The area of the rectangle is {} square pixels.",
    area1(rect1)
    );
}

fn area1(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//---------------------------------------------------------------
struct Rectangle {
    width: u32,
    height: u32,
}

fn kankan3(){
    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.",
    area2(&rect));

    println!("The area of the rectangle is {} square pixels.",
             area2(&rect));
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
//-------------------------------------------------------------
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn kankan4(){
    let rect = Rect{width: 30, height: 60};

    println!("rect is {:#?}",rect);
}

