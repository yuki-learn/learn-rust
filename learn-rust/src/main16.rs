use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    learn16_7();
}

fn learn16_1() {
    // スレッドで動かしたい処理をクロージャで渡す
    let handle = thread::spawn(|| {
        for i in 1..10 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 別で立ち上げたスレッドの処理が終わるまで待つ。
    // ※メインスレッドの実行をブロックしている
    handle.join().unwrap();
}

fn learn16_2() {
    // moveクロージャを使ってあるスレッドのデータを別スレッドで使用することができる
    let vec = vec![1, 2, 3];

    // クロージャに外で使ってる変数の所有権を強制的に奪わせる。
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", vec);
    });

    handle.join().unwrap();
}

fn learn16_3() {
    // チャンネルを生成
    // (送信側, 受信側)を返す
    let (tx, rx) = mpsc::channel();

    // スレッドで送信機から受信機に送信。※送信機の所有権はmoveキーワードでスレッドが持っている
    thread::spawn(move || {
        let val = String::from("やあ");
        tx.send(val).unwrap();

        // send関数は所有権をムーブさせ、受信機側がvalの所有権を持つことになる。
        // println!("val is {}", val);
    });

    // メッセージを受け取る(Result型)
    // recvはメインスレッドをブロックする
    let received = rx.recv().unwrap();

    println!("受け取ったメッセージ: {}", received);
}

// 複数のメッセージを受信
fn learn16_4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // スレッドからやあ(hi from the thread)
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn learn16_5() {
    let (tx, rx) = mpsc::channel();

    // 送信機をクローンして同時に2つの送信機から受信するようにしている
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        // 君のためにもっとメッセージを(more messages for you)
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn learn16_6() {
    let m = Mutex::new(5);

    {
        // ロックを獲得 ※現在のスレッドをブロックするためロックが獲得できないと処理が止まったままになる
        // 中に入っているデータへの可変参照を取得できる
        let mut num = m.lock().unwrap();
        *num = 6; // 参照外しで値を書き換える
    } // MutexGuardはDropを実装しているからそこでアンロックされる

    println!("m = {:?}", m);
}

fn learn16_7() {
    // Arcを使ってマルチスレッドでも所有権をクローンして処理できるようにする
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
