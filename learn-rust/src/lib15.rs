pub mod lib15 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use List1::{Cons, Nil};

    #[derive(Debug)]
    pub enum List1 {
        Cons(i32, RefCell<Rc<List1>>),
        Nil,
    }

    impl List1 {
        pub fn tail(&self) -> Option<&RefCell<Rc<List1>>> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    }

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: 'a + Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
                // 警告: 割り当ての75％以上を使用してしまいました
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
                // 切迫した警告: 割り当ての90%以上を使用してしまいました
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 1.0 {
                // エラー: 割り当てを超えています
                self.messenger.send("Error: You are over your quota!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::BorrowMut, cell::RefCell};

    use super::lib15::{LimitTracker, Messenger};

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // selfは不変参照なの変更できない
            // sendはMessengerトレイトで定義されているので、&mutにもできない
            // RefCellは可変参照は1つになるようにしていて、2つ以上になると、実行時エラーになる
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow_mut()でベクタの不変参照を得ている
        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }
}
