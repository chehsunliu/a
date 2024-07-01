use std::cell::RefCell;
use std::collections::VecDeque;

struct MyQueue {
    stacks: [RefCell<VecDeque<i32>>; 2],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stacks: [RefCell::new(VecDeque::new()), RefCell::new(VecDeque::new())],
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(v) = self.stacks[1].borrow_mut().pop_back() {
            self.stacks[0].borrow_mut().push_back(v);
        }
        self.stacks[0].borrow_mut().push_back(x);
    }

    fn pop(&mut self) -> i32 {
        while let Some(v) = self.stacks[0].borrow_mut().pop_back() {
            self.stacks[1].borrow_mut().push_back(v);
        }

        self.stacks[1].borrow_mut().pop_back().unwrap()
    }

    fn peek(&self) -> i32 {
        while let Some(v) = self.stacks[0].borrow_mut().pop_back() {
            self.stacks[1].borrow_mut().push_back(v);
        }

        *self.stacks[1].borrow().back().unwrap()
    }

    fn empty(&self) -> bool {
        self.stacks[0].borrow().is_empty() && self.stacks[1].borrow().is_empty()
    }
}
