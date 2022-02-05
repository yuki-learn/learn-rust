fn main() {
    learn_if();
    learn_loop();
}

// fn learn1() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6; // rustの
//     変数はデフォルトでイミュータブル。コンパイルエラーになる。
//     println!("The value of x is: {}", x);

//     // ミュータブルな変数を宣言する場合は「mut」を使用
//     let mut y = 5;
//     println!("The value of y is: {}", y);
//     y = 6; // 再代入可能
//     println!("The value of y is: {}", y);

//     // 定数は「const」で定義する。定数は型を明示的にする必要あり。
//     const MAX_VALUE: i32 = 100_000;

//     // シャドーイング
//     // rustは前に宣言した変数と同じ名前の変数を新しく宣言することが可能
//     // 変数をミュータブルにするのと違い、新しい変数を宣言している。前に宣言した変数の値を使用してミュータブルな新しい変数を宣言。
//     let a = 10;

//     let a = a * 2;

//     let a = a / 2;

//     // 新しい変数を生成しているので以下のようなことが可能

//     // 文字列型で宣言しているが、同じ名前の数値型の変数を宣言できる
//     let spaces = "   ";
//     let spaces = spaces.len();

//     // mutで宣言した変数は値は可変だが、定義した変数の型は可変ではないためエラーになる
//     let mut spaces = "   ";
//     spaces = spaces.len(); // error
// }

// fn learn2() {
//     // rustの型

//     // 整数型 i: 符号付きnビット, u: 符号なしnビット
//     // isize, usizeはそのマシンのポインタサイズと同じサイズの整数型(64bitアーキテクチャなら64bitになる)
//     let x: i8 = 1;
//     let x: i16 = 1;
//     let x: i32 = 1; // 整数型の基準(C#のintみたいな)。基本的にこれを使用
//     let x: i64 = 1;
//     let x: isize = 1;

//     let x: u8 = 1;
//     let x: u16 = 1;
//     let x: u32 = 1;
//     let x: u64 = 1;
//     let x: usize = 1;

//     // 浮動小数点型。f32とf64の2種類
//     let x: f32 = 1.0; // 単精度浮動小数点数
//     let x: f64 = 1.0; // 倍精度浮動小数点数

//     // 論理値型
//     let t = true;
//     let f = false;

//     // 文字型(char)
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻'; //ハート目の猫

//     // 複合型

//     // タプル
//     let tuple: (i32, f64, u8) = (500, 6.4, 1);

//     // 分解して取り出す
//     let (a, b, c) = tuple;

//     // 値のインデックスを指定して値を取り出す
//     println!("{}", tuple.0);
//     println!("{}", tuple.1);
//     println!("{}", tuple.2);

//     // 配列型

//     // Rustの配列は「固定長」で宣言後、要素の数を変えることができない。
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }

fn learn3() {
    another_function(10, 100);
    another_function2();
    println!("{}", five());
}

fn learn_if() {
    // if式
    let number = 4;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 式なので値を評価できる -> 変数に束縛できる。
    // if式で値を返す際は、同じ型でなければならない
    let num = if true { 10 } else { 20 };
}

fn learn_loop() {
    // rustにはloop, while, forがある

    // loop: 手動で止めるか、breakを使わないと無限ループ
    loop {
        println!("again!");
        break;
    }

    // while: 条件付きループ
    let mut num = 3;
    while num < 10 {
        num += 1;
    }
    println!("{}", num);

    // for
    let a = [10, 20, 30, 40, 50];
    // iter()を使って要素を走査
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// rustの関数名はスネークケースで定義する
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

fn another_function2() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// 戻り値は`-> 型名`で定義する
fn five() -> i32 {
    // Scalaなどと同じで関数内の最後の式を暗黙的に返す(returnは早期リターンなどで使う)
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; セミコロンを付けると式から文になりエラー(文は値に評価されない)
}
