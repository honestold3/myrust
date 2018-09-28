pub fn kankan(){
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    println!("{:?}",s);

    let mut st = String::from("foo");
    st.push_str("bar");
    println!("{:?}",st);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {}",s1);
    println!("s2 is {}",s2);
}

pub fn kankan1(){
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{:?}",s3);
}

pub fn format(){
    let s1 = String::from("tic");

    let s2 = String::from("tac");

    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}",s)
}

pub fn split_str(){
    let len = String::from("Hola").len();
    //let hello = "Здравствуйте";
    let hello = "你好";
    let s = &hello[0..6];
    println!("{:?}",s);

    for c in hello.chars(){
        println!("{}",c);
    }

    println!("-------------------------");

    for b in hello.bytes(){
        println!("{}",b);
    }

    println!("--------------------------");

    let v = vec![229,165,180];
    let kk = String::from_utf8_lossy(&v);
    println!("kk is {}",kk);

}