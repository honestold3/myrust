//extern crate iter_proj;

fn main() {
    //kankan();
    kankan1();
}

fn kankan(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("get: {}",val);
    }

    let mut v11_iter = v1.iter();
    assert_eq!(v11_iter.next(), Some(&1));

    assert_eq!(v11_iter.next(), Some(&2));

    assert_eq!(v11_iter.next(), Some(&3));

    assert_eq!(v11_iter.next(), None);

    let v2 = vec![1,2,3];
    let mut v2_iter = v2.into_iter();
    println!("kk2:{:?}",v2_iter.next());

    let v3 = vec![1,2,3];
    let mut v3_iter = v3.into_iter();
    println!("kk3:{:?}",v3_iter.next());
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total1 = v2_iter.count();
    assert_eq!(total1, 3);
}

fn kankan1(){
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}",v2);
}

//---------------------------------------------------------------
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let mut shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    //shoes[0].style = String::from("free"); //error ownership

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}