fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //let s = String::from("hehe");

    //change(&s); //error cannot borrow immutable borrowed content

    let mut s2 = String::from("hehe");

    change1(&mut s2);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change1(some_string: &mut String) {
    some_string.push_str(", haha");
}

