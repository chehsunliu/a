struct KthLargest {
    k: i32,
    counts: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut counts = vec![0; 20001];
        for v in nums {
            counts[(v + 10000) as usize] += 1;
        }
        Self { k, counts }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.counts[(val + 10000) as usize] += 1;

        let mut counts = 0;
        for v in (0..self.counts.len()).rev() {
            if self.counts[v] == 0 {
                continue;
            }

            counts += self.counts[v];
            if counts >= self.k {
                return v as i32 - 10000;
            }
        }

        unreachable!();
    }
}

// 2 4 4 5 5 8

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {}
}
