use std::collections::HashMap;
use std::cell::RefCell;


pub fn kankan(){
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("yellow"),50);
    println!("{:?}",scores);

    let teams = vec![String::from("kk"),String::from("kkk")];
    let init_scores = vec![111,222];
    let map: HashMap<_,_> = teams.iter().zip(init_scores.iter()).collect();
//    {
//        map.entry(&String::from("kk")).or_insert(&{ 50 });
//    }
    println!("{:?}",map)
}

pub fn kankan11(){

    let teams = vec![String::from("kk"),String::from("kkk")];
    let init_scores = vec![RefCell::new(111),RefCell::new(222)];
    let mut map: HashMap<_,_> = teams.iter().zip(init_scores.iter()).collect();

    let v = map.get(&String::from("kk")).unwrap();
    let aa = *v.borrow_mut();

    let bb = *v.borrow_mut();
    //change(v);

    println!("{:?}",map);
}

fn change<'a>(m: &'a RefCell<i32>){
    *m.borrow_mut() += 10;
}

pub fn kankan1(){
    let field_name = String::from("Favorite color");

    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    //println!("{}",field_name) error ownership
}

pub fn kankan2(){
    let mut map = HashMap::new();
    map.insert(String::from("Blue"),10);
    map.insert(String::from("yellow"),50);

    let name = String::from("Blue");
    let ss = map.get(&name);

    println!("{:?}",ss);
}

pub fn kankan3(){
    let mut map = HashMap::new();
    map.insert(String::from("Blue"),10);
    map.insert(String::from("yellow"),50);

    for (key, value) in &map {
        println!("{}: {}",key, value);
    }

}

pub fn kankan4(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);

    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let cc = scores.entry(String::from("Yellow")).or_insert(0);

    *cc = 55;

    println!("{}",cc)
}

pub fn kankan5(){
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {

        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{:?}", map);
}



