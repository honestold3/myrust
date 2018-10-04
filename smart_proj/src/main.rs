use List::{Cons, Nil};
use std::rc::Rc;

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

fn main() {
    kankan();
}
