pub fn kankan(){
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    println!("{:?}",v);

    let v1 = vec![1,2,3,4];
    //v1.push(5); error v1 is imutable

    println!("{:?}",v1);

    let third: &i32 = &v1[2];
    let third1: Option<&i32> = v1.get(2);

    println!("{:?},{:?}",third,third1)
}

pub fn kankan1(){
    let v = vec![1,2,3];
    for i in &v {
        println!("{}",i);
    }
}

pub fn kankan2(){
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}",v);
}

pub fn kankan3(){
    #[derive(Debug)]
    enum SpreadsheetCell {

        Int(i32),

        Float(f64),

        Text(String),

    }

    let row = vec![

        SpreadsheetCell::Int(3),

        SpreadsheetCell::Text(String::from("blue")),

        SpreadsheetCell::Float(10.12),

    ];

    println!("{:?}", row);
}