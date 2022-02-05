// モジュール front_of_house
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();

        // 「super」で今いるモジュールの親と同じ位置にいるモジュールにアクセスすることができる。
        super::serve_order();
    }

    fn cook_order() {}

    // 構造体も公開できる
    pub struct Breakfast {
        // 構造体のフィールドはデフォルトでprivateなため、pubで公開設定する必要がある
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enumの場合はデフォルトで要素が全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// pub useで外からもhostingで参照できるようになる。
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    // useで「front_of_house::hosting」を持ち込んでいる
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// トレイト境界構文でジェネリックで書くものと同じ糖衣構文
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 引数が2つ以上の場合、糖衣構文を使うことで「同じ型」じゃなくても使用できる関数を定義できる
// notify3は「Summary」を実装していれば同じ型でなくてもいいが、notify4は「Summaryを実装している同じ型」である必要がある
pub fn notify3(item: &impl Summary, item2: &impl Summary) {}
pub fn notify4<T: Summary>(item: T, item2: T) {}
