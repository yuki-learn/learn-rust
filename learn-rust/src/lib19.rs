use std::{fmt, ops::Add};

struct Counter;

impl Iterator for Counter {
    // Itemはプレースホルダー型
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        None
    }
}

struct Millimeters(u32);
struct Meters(u32);

// RHS型引数の値をMetersにセット
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
