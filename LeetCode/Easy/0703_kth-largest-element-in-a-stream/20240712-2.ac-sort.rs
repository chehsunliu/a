struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self { k, nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort();
        self.nums[self.nums.len() - self.k as usize]
    }
}

// 2 3 4 5 5 8 8 8 10

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {}
}
