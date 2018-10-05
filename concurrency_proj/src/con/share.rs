use std::sync::{Mutex, Arc};
use std::thread;

pub fn kankan(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

//    let mut num2 = m.lock().unwrap();
//    *num2 = 6;
//    drop(num2);

    println!("m = {:?}", m);
}

pub fn kankan1(){
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);

//    let handle2 = thread::spawn(move || {
//        let mut num2 = counter.lock().unwrap();
//        *num2 += 1;
//    });
//    handles.push(handle2);
}

pub fn kankan2(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}