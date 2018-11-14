enum IpAddKind {
    V4,
    V6,
}

fn kankan(){
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;
}

fn route(ip_type: IpAddKind) {
    return
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

fn kankan1(){
    route(IpAddKind::V4);
    route(IpAddKind::V6);

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("192.1168.1.1"),
    };
    let loopback = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("192.1168.1.1"),
    };

    enum Ipaddr {
        V4(String),
        V6(String),
    }
    let home1 = Ipaddr::V4(String::from("127.0.0.1"));
    let loopback = Ipaddr::V6(String::from("::1"));
}

fn kankan2(){
    #[derive(Debug)]
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }
    let home = IpAddr::V4(127,0,0,288);// 288超过u8的长度
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}",home)
}

//---------------------adv ----------------
#[derive(Debug)]
enum Message<'a> {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage<'a>),
    Write1(WriteMessage1),
    ChangeColor(ChangeColorMessage),
}
#[derive(Debug)]
struct QuitMessage; // unit struct

#[derive(Debug)]
struct MoveMessage { x: i32, y: i32, }

#[derive(Debug)]
struct WriteMessage<'a>(&'a str); // tuple struct

#[derive(Debug)]
struct WriteMessage1(String); // tuple struct

#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl<'a> Message<'a> {
    fn call(&self){
        match *self {
            Message::Quit(_) => println!("quit"),
            Message::Move(_) => println!("move"),
            Message::Write(WriteMessage(d)) => {
                println!("write is {}",d);
            },
            Message::Write1(WriteMessage1(ref d)) => {
                println!("write1 is {}",d);
            },
            Message::ChangeColor(ChangeColorMessage(a,b,c)) => {
                println!("change,{},{},{}",a,b,c);
                //kk(ChangeColorMessage(a,b,c));
            },
        }
    }
}

fn kankan3(){
    let q = WriteMessage("hello");
    let m = Message::Write(q);
    m.call();

    let q1 = WriteMessage1(String::from("hello1"));
    let m1 = Message::Write1(q1);
    m1.call();

    let change = Message::ChangeColor(ChangeColorMessage(11,22,33));
    change.call();
}

fn kk(c: ChangeColorMessage) {
    println!("{:?}",c)
}

//-----------------------------------------------------------

fn kankan4(){
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // 特别有意思的是：
    match (*reference)*2 {
        val => println!("Got a value via dereferencing: {:?}", val), //=> 4*2=8
    }

    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}

//----------------------- option---------------------
fn kankan_option(){
    let x: i32 = 3;
    let y: Option<i32> = Some(5);
    let k = plus(x,y);
    println!("plus:::{:?}",k)
}

fn plus(x: i32,y: Option<i32>) -> Option<i32> {
    match y {
        None => None,
        Some(i) => Some(i+x),
    }
}

fn main() {
    //kankan();
    //kankan1();
    //kankan2();
    //kankan3();
    kankan4();
    //kankan_option();
}





