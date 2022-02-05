fn main() {
    learn9();
}

fn learn1() {
    // let ifを使用
    // ※matchと違い、コンパイラが網羅性を確認してくれない
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        // あなたのお気に入りの色、{}を背景色に使用します
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        // 火曜日は緑の日！
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            // 紫を背景色に使用します
            println!("Using purple as the background color");
        } else {
            // オレンジを背景色に使用します
            println!("Using orange as the background color");
        }
    } else {
        // 青を背景色に使用します
        println!("Using blue as the background color");
    }
}

fn learn2() {
    let x = 2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    };
}

fn learn3() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 外側のスコープの「y」ではなく、Optionの中身を取り出すために新しくyを定義している
        // このmatch式はxについてなので「y = 5」
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    };

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn learn4() {
    let x = 1;

    match x {
        // 「|」でOrを表現できる
        1 | 2 => println!("One or Two"),
        3 => println!("Three"),
        _ => println!("anything"),
    };

    let x = 3;
    match x {
        // 「...」で範囲にマッチさせることもできる
        1...5 => println!("one through five"),
        _ => println!("anything"),
    };
}

struct Point {
    x: i32,
    y: i32,
}

fn learn5() {
    let p = Point { x: 10, y: 20 };

    // 構造体を分解できる。
    let Point { x: a, y: b } = p;

    println!("a = {}, b = {}", a, b);

    let p = Point { x: 0, y: 7 };

    match p {
        // x軸上の{}
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // y軸上の{}
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // どちらの軸上でもない: ({}, {})
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn learn6() {
    // enumでパターンマッチ
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            // Quit列挙子には分配すべきデータがない
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                // x方向に{}、y方向に{}だけ動く
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        // テキストメッセージ: {}
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                // 色を赤{}, 緑{}, 青{}に変更
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

fn learn7() {
    let tuple = (1, 2, 3, 4, 5);

    // 全ての要素に「_」をつけなくても「..」を使って残りの要素全て無視することができる
    match tuple {
        (1, 2, ..) => println!("ok"),
        _ => println!("ng"),
    }

    // 最初と最後だけを取り出す方法
    match tuple {
        (first, .., last) => println!("ok"),
        _ => println!("ng"),
    }

    // これは値が特定できないため、コンパイルエラーになる
    // match tuple {
    //     (.., second..) => println!("ok"),
    //     _ => println!("ng"),
    // }
}

fn learn8() {
    let robot_name = Some(String::from("Bors"));

    // match robot_name {
    //     // nameにrobot_nameの所有権が1部ムーブされてしまったので最後の出力がエラーになる
    //     Some(name) => println!("{}", name),
    //     None => (),
    // }

    match robot_name {
        // パターンで参照を生成する際はrefキーワード
        // これで、robot_nameから所有権は移動せず、参照を取っただけになる
        // ref mutにすれば可変参照
        Some(ref name) => println!("{}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}

fn learn9() {
    let num = Some(10);

    match num {
        Some(x) if x % 2 == 0 => println!("x is even"),
        Some(x) => (),
        None => (),
    }
}
