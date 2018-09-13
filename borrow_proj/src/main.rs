fn main() {
    //kankan()
    //kankan1()
    kankan2()
}

fn kankan(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{} is {}'",s1,len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//-------------------------------------

fn kankan1(){
    let s = String::from("hello");
    change(&s); //error is s imm
}

fn change(some_string: &String) {
    //some_string.push_str(", world");
}
//-----------------------------------

fn kankan2(){
    let mut s = String::from("hello");
    change1(&mut s);
    println!("s is {}",s);
}

fn change1(some_string: &mut String) {
    some_string.push_str(", world");
}