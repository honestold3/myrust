use std::slice;

pub fn kankan(){
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let a  = String::from("dd");

    println!("{:?}",&a);

    unsafe {
        println!("*r1 is: {:?}", *r1);
        println!("*r2 is: {:?}", *r2);

        println!("&r1 is: {:?}", r1);
        println!("&r2 is: {:?}", r2);
    }
}

pub fn kankan1(){
    let address = 0x012345usize;
    let r = address as *const i32;

    //println!("{:?}",*r); //error raw pointer
    unsafe {
        println!("{:?}", *r);
    }
}

//-------------------------------------------------
pub fn kankan2(){
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    //let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut1(r,3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

//fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//    let len = slice.len();
//    assert!(mid <= len);
//    (&mut slice[..mid],
//     &mut slice[mid..])
//}


fn split_at_mut1(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

pub fn kankan3(){
    let address = 0x012345usize;
    let r = address as *mut i32;
    let slice = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };

    println!("{:?}",slice)
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn kankan4() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";

pub fn kankan5() {
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn kankan6() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}