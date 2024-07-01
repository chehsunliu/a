use std::cell::RefCell;
use std::collections::VecDeque;

struct MyStack {
    q1: RefCell<VecDeque<i32>>,
    q2: RefCell<VecDeque<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self {
            q1: RefCell::new(VecDeque::new()),
            q2: RefCell::new(VecDeque::new()),
        }
    }

    fn push(&mut self, x: i32) {
        if !self.q2.borrow().is_empty() {
            self.q2.borrow_mut().push_back(x);
        } else {
            self.q1.borrow_mut().push_back(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if !self.q1.borrow().is_empty() {
            while self.q1.borrow().len() > 1 {
                let v = self.q1.borrow_mut().pop_front().unwrap();
                self.q2.borrow_mut().push_back(v);
            }
            self.q1.borrow_mut().pop_front().unwrap()
        } else {
            while self.q2.borrow().len() > 1 {
                let v = self.q2.borrow_mut().pop_front().unwrap();
                self.q1.borrow_mut().push_back(v);
            }
            self.q2.borrow_mut().pop_front().unwrap()
        }
    }

    fn top(&self) -> i32 {
        if !self.q1.borrow().is_empty() {
            while self.q1.borrow().len() > 1 {
                let v = self.q1.borrow_mut().pop_front().unwrap();
                self.q2.borrow_mut().push_back(v);
            }
            let v = self.q1.borrow_mut().pop_front().unwrap();
            self.q2.borrow_mut().push_back(v);
            return v;
        } else {
            while self.q2.borrow().len() > 1 {
                let v = self.q2.borrow_mut().pop_front().unwrap();
                self.q1.borrow_mut().push_back(v);
            }
            let v = self.q2.borrow_mut().pop_front().unwrap();
            self.q1.borrow_mut().push_back(v);
            return v;
        }
    }

    fn empty(&self) -> bool {
        self.q1.borrow().is_empty() && self.q2.borrow().is_empty()
    }
}
