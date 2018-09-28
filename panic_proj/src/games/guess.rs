

use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

pub fn guess_num(){
    println!("Guess the number!");

    //let secret_number = rand::thread_rng().gen_range(1, 101);
    let secret_number = thread_rng().gen_range(1,101);

        loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //panic!("{} is not a number",guess);
                println!("{} is not a number",guess);
                continue
            },
        };

        println!("You guessed: {}", guess);

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//---------------------------------------------------------------------------

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }

    pub fn new1(value: u32) -> Guess {
        if value < 1 || value > 100 {
            //panic!("Guess value must be between 1 and 100, got {}.", value);
            println!("Guess value must be between 1 and 100, got {}.", value);
            //continue;
            Guess {
                value: 111,
            }
        } else {
            Guess {
                value
            }
        }

    }

    pub fn value(&self) -> u32 {
        self.value
    }

}

pub fn guess_num1(){
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = Guess::new(guess.trim().parse().expect("Not a number!"));

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}