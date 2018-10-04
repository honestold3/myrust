

pub fn kankan(){
//     let r;                       // -------+-- 'a
//     {                            //        |
//         let x = 5;               // -+-----+-- 'b
//         r = &x;                  //  |     |
//     }                            // -+     |
//     println!("r: {}", r);        //        |
//                                  //--------+
 }


pub fn kankan1(){

     let x =5;      //-------+--'b
                         //       |
     let r = &x;   //--+----+--'a
                         //  |    |
     println!("r: {}",r);//  |    |
                         //--+    |
 }                       //-------+

pub fn kankan3(){
    let string1 = String::from("hjl");
    let string2 = "asd";
    let result = longest(string1.as_str(),string2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn kankan4() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    println!("{:?}",i);
    println!("{:?}",i.level());
    println!("{:?}",i.announce_and_return_part("hello lifetime"));

    let sss: &'static str = "I have a static lifetime.";
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}",ann);
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

pub fn kankan5(){
    let s1 = String::from("hello");
    let s2 = "Life time";
    let s3 = longest_with_an_announcement(&s1,&s2,"kkkkkk");
    println!("{}-{}-{}",s1,s2,s3);
}

