use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    sample3();
}

fn sample1() {
    // panic!("crash and burn"); // 回復不能エラー
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // 見つからない場合は作る
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                }
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}

fn sample2() {
    // unwrap()で冗長なmatchを書かなくてもOkなら中身を返し、Errならpanic!を呼んでくれる
    let f = File::open("hello.txt").unwrap();
}

fn sample3() {
    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // ?を使うと、match式で返したような動きをする
    // OkのときはOkの中身が、ErrのときはErrが返却される
    // ※「?」はResultを返す関数内でしか使用できない。
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
