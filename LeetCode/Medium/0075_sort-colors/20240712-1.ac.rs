impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut heap = MinHeap::new();
        for v in nums.iter() {
            heap.insert(*v);
        }

        println!("{:?}", heap.values);

        for i in 0..nums.len() {
            nums[i] = heap.pop().unwrap();
        }
    }
}

struct MinHeap {
    values: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn insert(&mut self, value: i32) {
        self.values.push(value);

        let mut i: usize = self.values.len() - 1;
        while i > 0 {
            let parent_i = (i - 1) / 2;
            if self.values[parent_i] > self.values[i] {
                self.values.swap(parent_i, i);
                i = parent_i;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.values.len() <= 1 {
            return self.values.pop();
        }

        let value = self.values[0];
        self.values[0] = self.values.pop().unwrap();

        let mut i = 0;
        while i < self.values.len() {
            let i_l = i * 2 + 1;
            let i_r = i * 2 + 2;

            let i_larger = if i_l < self.values.len() && self.values[i_l] < self.values[i] {
                i_l
            } else {
                i
            };
            let i_larger = if i_r < self.values.len() && self.values[i_r] < self.values[i_larger] {
                i_r
            } else {
                i_larger
            };

            if i != i_larger {
                self.values.swap(i, i_larger);
                i = i_larger;
            } else {
                break;
            }
        }

        Some(value)
    }
}

//    0
//  1   2
// 3 4 5 6

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {
        Solution::sort_colors(&mut vec![
            1, 1, 1, 0, 1, 1, 0, 0, 2, 0, 0, 1, 1, 2, 1, 1, 1, 2, 0, 0, 2, 0, 2, 1, 1, 0, 1, 0, 0,
            1, 0, 2, 1, 2, 2, 2, 1, 0,
        ]);
    }
}
