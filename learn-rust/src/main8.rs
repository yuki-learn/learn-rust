use std::collections::HashMap;

fn main() {
    // ベクタ コレクション
    let v: Vec<i32> = Vec::new();

    // vec!で与えた値を保持するベクタを生成できる
    // ベクタはスコープを抜けると中身もドロップする
    let v2 = vec![1, 2, 3];

    // mutで中身を可変にできる
    let mut v3 = Vec::new();
    v3.push(3);
    v3.push(5);
    v3.push(7);

    let third: &i32 = &v2[2]; // 添字でアクセス
    let third: Option<&i32> = v2.get(2); // getメソッドでアクセス

    for num in &v2 {
        println!("{}", num);
    }

    let data = "initial contents";

    // 文字列リテラルからStringを生成
    let s = "initial contents".to_string();

    let hello = "Здравствуйте";
    let a = &hello[0..4];
    println!("{}", a);

    // HashMap
    // ベクタと同様にデータはヒープに保持される
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let a = scores.get("Blue");

    for (key, value) in &scores {
        println!("{}={}", key, value);
    }
}
