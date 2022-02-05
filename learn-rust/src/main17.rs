mod lib17;

use lib17::{Button, Draw, Screen};

use crate::lib17::Post;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        //セレクトボックスを実際に描画するコード
    }
}

impl Draw for String {
    fn draw(&self) {}
}

fn main() {
    // learn1();
    learn2();
}

fn learn1() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    // はい
                    String::from("Yes"),
                    // 多分
                    String::from("Maybe"),
                    // いいえ
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                // 了解
                label: String::from("OK"),
            }),
            Box::new(String::from("aaa")),
        ],
    };

    screen.run();
}

fn learn2() {
    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
