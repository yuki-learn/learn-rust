enum List {
    // Boxを使うことで無限にあるわけではないことをコンパイラに教える
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

impl<T> Deref for MyBox<T> {
    type Target = T; // Derefトレイトが使用する関連型を定義

    fn deref(&self) -> &T {
        // *演算子でアクセスしたい値への参照
        // これでMyBox<T>を参照外ししたときの参照がわかり、この参照を外せばいいということが定義できる
        &self.0 // *演算子使うとこれの参照を外して値を返す
    }
}

use List::{Cons, Nil};
mod lib15;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    v.iter()
        .filter(|&x| x % 2 == 0)
        .for_each(|x| println!("{}", x));

    // i32をヒープに格納
    let b = Box::new(10);
    println!("b = {}", b);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 参照外し演算子

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // yはxの参照なので、数値と参照は比較できず、5と比較するなら参照外し演算子で参照が示す値まで辿る必要がある

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Derefトレイトを実装しないとMyBoxを参照外しする方法がわからないからエラーとなる
    assert_eq!(5, *(y.deref())); // ↑と同じ

    let m = String::from("Rust");
    let m2 = MyBox::new(String::from("Rust"));
    hello(&m);
    // MyBox<String>の参照を渡しているが、Derefを実装しているため「参照外し型強制」が働き、&Stringに変換される
    // StringはDerefを実装していて&strに変換される
    hello(&m2);

    // Rc<T>

    // aのリストを保持するb, cを定義したい
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    // let b = Cons(3, Box::new(a)); // aはbが所有している
    // let c = Cons(4, Box::new(a)); // bにムーブしているからaはもう使えない

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rcを使って所有権を奪うのではなくクローンしている
    // クローンするたびに共有する参照の数は増え、参照が0個にならないとデータは破棄できない
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    check_rc_count();

    println!("------learn15_6-----\n");
    learn15_6();
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Rc<T>を使ってクローンした参照の数の推移を見る
fn check_rc_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // a生成後のカウント = {}
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    // b生成後のカウント = {}
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        // c生成後のカウント = {}
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // cがスコープを抜けた後のカウント = {}
    // Dropトレイトの実装が自動的に参照カウントを減らしている。
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn learn15_5() {
    // RefCell<T>
    // 参照やBox<T>などは借用規則の不変条件がコンパイル時に強制されているが、RefCell<T>は実行時に強制される
    // スレッドセーフでないのでマルチスレッドで使用できない

    // 通常、不変値は可変で借用することができない
    let x = 5;
    // let y = &mut x;
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // 親ノードを参照できるけど、所有権は持たない
    children: RefCell<Vec<Rc<Node>>>,
}

// 循環参照と循環参照を回避する
fn learn15_6() {
    use lib15::lib15::List1;

    let a = Rc::new(List1::Cons(5, RefCell::new(Rc::new(List1::Nil))));

    // aの最初の参照カウント = {}
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // aの次の要素は = {:?}
    println!("a next item = {:?}", a.tail());

    // aの所有権をコピーしたものを含むリストを作成
    let b = Rc::new(List1::Cons(10, RefCell::new(Rc::clone(&a))));

    // b作成後のaの参照カウント = {}
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // bの最初の参照カウント = {}
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // bの次の要素 = {:?}
    println!("b next item = {:?}", b.tail());

    // tail()はOptionが返る
    // aの次の要素を取得してきて、可変参照を得て(Rc<List1>)それをbに入れ替える
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // aを変更後のbの参照カウント = {}
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // aを変更後のaの参照カウント = {}
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // 次の行のコメントを外して循環していると確認してください; スタックオーバーフローします
    // println!("a next item = {:?}", a.tail()); // aの次の要素 = {:?}

    // Week<T>で循環参照を回避する
    // 弱い参照と呼ばれ、Rc<T>はstrong_countが0になったときインスタンスが破棄されるけど、
    // Week<T>はインスタンスの破棄にカウントが0である必要がない。

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // upgradeで親の参照が生きているかどうかを確認する必要がある。ここでは親が入っていないので空
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 親の可変参照を受け取って参照外しする。downgradeを使ってRc -> Weekに変換し、親を入れる
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

struct SampleDrop {
    data: String,
}

impl Drop for SampleDrop {
    fn drop(&mut self) {
        println!("{}はDropした", self.data);
    }
}
