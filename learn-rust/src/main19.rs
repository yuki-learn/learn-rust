use core::fmt;

use lib19::OutlinePrint;

mod lib19;

fn main() {
    learn6();
}

fn learn1() {
    let mut num = 5;

    // ※「*」は参照外しではない
    // safeでも生ポインタは生成可能。できないのは参照外しすること
    // 不変参照と可変参照は同時に生成できないが生ポインタならできる
    let r1 = &num as *const i32; // 不変な生ポインタ
    let r2 = &mut num as *mut i32; // 可変な生ポインタ

    unsafe {
        // 生ポインタの参照外し
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// ## 明確化のためのフルパス記法: 同じ名前のメソッドを呼ぶ
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // 子犬
        String::from("puppy")
    }
}

fn learn2() {
    // Animal::baby_name()はメソッドではなく、関連関数で、selfなく、どのAnimal::baby_name()か推論できずエラーになる
    // println!("A baby dog is called a {}", Animal::baby_name());

    // フルパス記法で、Dog型をAnimalとして扱う。と教えてあげる必要がある
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn learn3() {
    let p = Point { x: 10, y: 2 };
    println!("{}", p);
}

// Vec<T>にDisplayを実装したいが、クレートの外で定義されているので
// 直接Displayを実装することはできない。
// ニュータイプパターンを使用して、Vec<T>をラップする構造体を作り、それにDisplayを実装するようにする
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", ")) // タプル(Vec<String>)の中身を使用
    }
}

fn learn4() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn learn5() {
    // 型エイリアス -> 既存の型に別名を与える
    type Kilometers = i32;

    let a: i32 = 10;
    let b: Kilometers = 10;

    // 実態はi32なのでそのまま計算できる
    println!("{}", a + b);

    // メリットとしては、型にわかりやすい名前が付けられる点と、以下のように長い型の記述を短くすることができる
    type Thunk = Box<Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    let takes_long_type = |f: Thunk| {};
    let returns_long_type = |f: Thunk| {};
}

// never型: 絶対に値が返らない
// fn bar() -> ! {
//     // --snip--
// }

// 関数ポインタ
// 関数に匿名関数を渡せるけど、普通に定義した関数も渡すことができる

fn add_one(x: i32) -> i32 {
    x + 1
}

// クロージャと異なり、fnはトレイトではなく、型になる
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// クロージャはトレイトによって表現できるのでfnを使って型として値を返却することができない
// -> コンパイラがクロージャをどれくらいスペースを必要としているのかがわからないから。
// Boxで包むことでコンパイルできるようになる
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn learn6() {
    println!("{}", do_twice(add_one, 10));
    println!("{}", do_twice(|x: i32| x + 10, 10));
}
