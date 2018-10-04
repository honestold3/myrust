use std::thread;
use std::time::Duration;

fn main() {
    //kankan();
    //kankan_closure();
    //kankan1();
    //kankan2();
    kankan3()
}

fn kankan(){
    let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));
    println!("{}",equal_to_x(y));

    //-----------------------------------
    //fn add_one_v1(x: u32) -> u32 { x + 1 };

    //let add_one_v2 = |x: u32| -> u32 { x + 1 };

    //let add_one_v3 = |x|{ x+1 };

    //let add_one_v4 = |x| x + 1;

}

fn kankan_closure(){
    let example_closure = |x|x;
    let s  = example_closure(String::from("hello"));
    println!("{}",s);

    //let n = example_closure(5); //如果尝试对同一闭包使用不同类型则会得到类型错误
}

fn kankan1(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn kankan2(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout2(simulated_user_specified_value, simulated_random_number);
}

fn kankan3(){
    let x = vec![1, 2, 3];
    //let equal_to_x = move |z| z == x;  //error ownership
    let equal_to_x = |z| z == x;
    println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result

        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}

//-----------------------------------------------------------------------
fn generate_workout1(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

//----------------------------------------------------------------------
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout2(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}



#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 1);
}
