use learn_rust;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    learn_rust::eat_at_restaurant();
    learn_rust::hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret_number);

    let mut h = HashMap::new();
    h.insert("Ashley", "645-7689");
}
