// 所有権
// プログラムはメモリの使い方を管理する必要があり、以下のような方法がある
// * 定期的に使用されていないメモリを探す、ガベージコレクションを持つ言語
// * プログラマが明示的にメモリを確保したり、開放したりする必要がある言語

// rustはこれらとは別の「所有権」というシステムでメモリを管理している。

// ## 所有権規則
// * Rustの各値は、所有者と呼ばれる変数と対応している。
// * いかなるときも所有者は1人である。
// * 所有者がスコープから外れたら、値は破棄される。

// ## String型
// String型はヒープにメモリを確保するためコンパイル時にはサイズが不明なテキストを定義できる。
// (※文字列リテラルでは不変であり、スタックに積まれる)

// ヒープにメモリを確保する要求をOSに出して、使い終わったらOSに返す必要がある
// ガベージコレクタ付きの言語は使用されていないメモリを検知して解放してくれるため意識する必要がない
// **Rustの場合、メモリを所有している変数がスコープを抜けたらメモリが自動的に返還されるようになっている。**
// `drop`というメモリを返還する関数があり、閉じ括弧などスコープが抜けた段階で`drop`が自動的に呼ばれる。
fn learn() {
    let s = String::from("hello"); // メモリを確保
    println!("{}", s);
} // スコープを抜けるのでここで`s`は終わり

fn learn2() {
    // xに5を束縛し、xの値をコピーしてyに束縛する。スタックに積まれどちらも5になる
    let x = 5;
    let y = x;

    // Stringの場合、ヒープのためs2には文字列の値を保持するポインタがコピーされる。ポインタが指すヒープ上のデータがコピーされるわけではない。
    // 変数のスコープを抜けると自動解放されるため、s1, s2が同じポインタを解放すると二重解放となりバグの1つとなるが、
    // Rustはs2にs1のポインタをコピーした段階で、s1が有効でないと判断し、解放されなくなる。以降、s2だけ有効になる。
    // (※`Copy`トレイトを実装していると古い変数も有効のまま。i32などスタックに積まれる型)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); ← 無効化された変数は参照できないため、エラーになる。

    // 関数に渡す場合
    let s = String::from("hello");

    learn2_1(s); // sの値が関数にムーブされ、`s`が無効になる
    let x = 5;

    learn2_2(x); // xも関数にムーブされるが、i32にはCopyなので`x`は使用可能

    // 関数が戻り値
}

fn learn2_1(s: String) {
    println!("{}", s);
}

fn learn2_2(x: i32) {
    println!("{}", x);
}

fn learn3() {
    // 値の所有権を渡さず、値の参照を渡すことができる。
    // その場合、所有権は動かないので、呼び出し元の変数は使えるままで、
    // 呼び出し先の関数も所有権を返す必要がない。

    let s1 = String::from("hello");

    // 関数に引数に参照を取ることを「借用」という
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn learn4() {
    let s = String::from("he llo");
    let result = first_word(&s);
    println!("{}", result);

    // 文字列スライス(不変な参照)
    let s2 = String::from("hello world");
    let hello = &s2[0..5]; // s2の一部の参照
    let world = &s2[6..11];
    println!("{0}, {1}", hello, world);

    // 文字列リテラルは文字列スライスである「&str」
}

// 文字列スライスを返す関数
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // 空白まで繰り返す
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn calculate_length(s: &String) -> usize {
    // s.push_str("hello"); 借用した参照は不変なのでエラーになる
    s.len()
}

fn calculate_length2(s: &mut String) {
    // 「&mut」を付けると可変な参照となる。呼び出し元では「&mut 変数名」で呼び出す。
    s.push_str("hello");
}

// fn dangle() -> &String {
//     // 無効な変数の参照を返すことができないようになっている。
//     let s = String::from("hello");

//     &s
// }

fn main() {
    learn4();
}
