/// `RefCell` and interior mutability
///
/// - can think of it as private mutability, but immutable to the outside world
///
/// enforcing borrow rules at runtime with `RefCell<T>`
/// - unliked `Rc`, the cell type represents single ownership over the data that it holds
/// - in order to make changes or read (methods) we need to go through its API:
///   - `borrow_mut`
///   - `borrow`
/// - we are limited to ONE MUTABLE BORROW

pub trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[allow(dead_code)]
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
        let mut message: &str = "";

        if percentage_of_max >= 1.0 {
            message = "Error: you are over your quota!";
        } else if percentage_of_max >= 0.9 {
            message = "Urgent warning: You've used up over 90% of your quota!";
        } else if percentage_of_max >= 0.75 {
            message = "Warning: You've used up over 75% of your quota!";
        }
        self.messenger.send(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

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
            self.sent_messages.borrow_mut().push(String::from(message));

            // invalid (two mutable borrows)
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messeger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messeger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messeger.sent_messages.borrow().len(), 1);
    }
}
