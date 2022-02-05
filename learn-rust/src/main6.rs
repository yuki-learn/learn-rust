// # enum
// rustのenumはHaskellなどの代数的データ型と似ている。

enum IpAddr {
    V4(String),
    V6(String),
}

fn plus(value: i32) -> i32 {
    value + 100
}

fn main() {
    let some_number = Some(10);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let result = map(plus, some_number);

    match result {
        None => println!("None"),
        Some(value) => println!("{}", value),
    }

    if let Some(3) = some_number {
        println!("ten");
    }
}

fn map<T, S>(func: fn(S) -> T, option: Option<S>) -> Option<T> {
    match option {
        None => None,
        Some(value) => Some(func(value)),
    }
}
