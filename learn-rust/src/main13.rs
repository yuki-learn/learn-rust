use core::num;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, S>
where
    T: Fn(U) -> S,
    U: Hash + Eq + Copy,
    S: Copy,
{
    calculation: T,
    hashmap: HashMap<U, S>,
}

impl<T, U, S> Cacher<T, U, S>
where
    T: Fn(U) -> S,
    U: Hash + Eq + Copy,
    S: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, S> {
        Cacher {
            calculation,
            hashmap: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> S {
        match self.hashmap.get(&arg) {
            Some(&n) => n,
            None => {
                let v = (self.calculation)(arg);
                self.hashmap.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // クロージャ(匿名関数)
    // 引数を||で囲む。複数引数のときはカンマ区切り -> |param1, param2|
    // |num: u32|のように型注釈も書ける
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2 = v1.iter().map(|x| x + 1);

    for num in v2 {
        println!("{}", num);
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
