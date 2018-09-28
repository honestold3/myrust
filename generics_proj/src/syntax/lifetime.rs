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