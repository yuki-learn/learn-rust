// 構造体
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド
// Rectangle構造体に関数を実装する。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn learn1() {
    // 可変な構造体のインスタンスは「mux」を付ける。※ただし、フィールド個別で可変にすることはできない。
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn main() {
    learn1();
}
