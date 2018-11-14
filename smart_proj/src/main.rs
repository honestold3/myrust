use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn kankan(){
//    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//    let b = Cons(3, Box::new(a));
//    let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

//----------------------------------------------------

fn kankan1(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//----------------------------------------------------------

fn kankan2(){
//    let x = 5;
//    let y = &mut x;
}

//-----------------------------------------------------------
#[derive(Debug)]
enum List1 {
    Cons1(Rc<RefCell<i32>>, Rc<List1>),
    Nil,
}

use List1::{Cons1,Nil as OtherNil};

fn kankan3(){
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons1(Rc::clone(&value), Rc::new(OtherNil)));
    let b = Cons1(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons1(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
//---------------------------------------------------------
use List2::{Cons2, Nil as oNil};

#[derive(Debug)]
enum List2 {
    Cons2(i32, RefCell<Rc<List2>>),
    Nil,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match *self {
            Cons2(_, ref item) => Some(item),
            oNil => None,
        }
    }
}

fn kankan4(){
    let a = Rc::new(Cons2(5, RefCell::new(Rc::new(oNil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons2(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //Uncomment the next line to see that we have a cycle; it will
    //overflow the stack
    //println!("a next item = {:?}", a.tail());
}
//----------------------------------------------------------
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn kankan5() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
//-------------------------------------------------------

fn kankan6() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
}

fn main() {
    //kankan();
    //kankan1();
    //kankan2();
    //kankan3();
    //kankan4();
    //kankan5();
    //kankan6();
    //kankan_refcell();
    kankan_borrow();
}

fn kankan_refcell(){
//    let mut data = 100_i32;
//    let p : &i32 = &data;
//    data = 10;
//    println!("{}", *p);

    use std::cell::Cell;

    let data : Cell<i32> = Cell::new(100);
    let p = &data;
    data.set(10);
    println!("{}", p.get());

    p.set(20);
    println!("{:?}", data);

}

//----------------------------------------
fn kankan_borrow(){
    let data = RefCell::new(5);
    demo(&data);
}

fn demo(r: &RefCell<i32>) {
    immut_borrow(&r.borrow());
    mut_borrow(&mut r.borrow_mut());
    mut_borrow(&mut r.borrow_mut());
    immut_borrow(&r.borrow())
}


fn immut_borrow(a: &i32){
    println!("a is {}",a)
}

fn mut_borrow(b: &mut i32){
    *b +=1;
}


