enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("is penny!");
            1
        } ,
        Coin::Nickel =>5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn kankan(){
    let c = Coin::Penny;
    value_in_cents(c);
}
//--------------------------------------
fn kankan1(){
    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("there"),
        _ => (), //placeholder
    }
}

//----------------------------------------

enum Kingdom {Plant(u32, &'static str),Animal(u32, &'static str)}

fn kankan2(){
// A list of data to search through.
    let all_the_big_things = [
        Kingdom::Plant(250, "redwood"),
        Kingdom::Plant(230, "noble fir"),
        Kingdom::Plant(229, "sugar pine"),
        Kingdom::Animal(25, "blue whale"),
        Kingdom::Animal(19, "fin whale"),
        Kingdom::Animal(15, "north pacific right whale"),
    ];

    // We're going to search for the name of the biggest animal,
// but to start with we've just got `None`.
    let mut name_of_biggest_animal = None;
    let mut size_of_biggest_animal = 0;
    for big_thing in &all_the_big_things {
        match *big_thing {
            Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
                // Now we've found the name of some big animal
                size_of_biggest_animal = size;
                name_of_biggest_animal = Some(name);
            }
            Kingdom::Animal(..) | Kingdom::Plant(..) => ()
        }
    }

    match name_of_biggest_animal {
        Some(name) => println!("the biggest animal is {}", name),
        None => println!("there are no animals :("),
    }
}

//--------------------------------------

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn kankan_match(){
    let mut count = 0;
    let coin = Coin1::Quarter(UsState::Alabama);
    match coin {
        Coin1::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn kankan_if_let(){
    let mut count = 0;
    let coin = Coin1::Quarter(UsState::Alaska);
    if let Coin1::Quarter(state) = coin {
        println!("State quarter from {:?}!",state);
    } else {
        count += 1;
    }
}

fn main() {
    //kankan()
    //kankan1()
    //kankan2()
    kankan_match();
    kankan_if_let();
}
