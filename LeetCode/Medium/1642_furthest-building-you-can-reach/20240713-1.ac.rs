use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for i in 0..(heights.len() - 1) {
            if heights[i] >= heights[i + 1] {
                continue;
            }

            let diff = heights[i + 1] - heights[i];
            if heap.len() < ladders as usize {
                heap.push(Reverse(diff));
                continue;
            }

            if let Some(&Reverse(diff_low)) = heap.peek() {
                if diff_low < diff {
                    let b = heap.pop().unwrap().0;
                    heap.push(Reverse(diff));

                    bricks -= b;
                    if bricks < 0 {
                        return i as i32;
                    }
                } else {
                    bricks -= diff;
                    if bricks < 0 {
                        return i as i32;
                    }
                }
            } else {
                bricks -= diff;
                if bricks < 0 {
                    return i as i32;
                }
            }
        }

        heights.len() as i32 - 1
    }
}

// 1 3 0 1 1 1
// 4 5

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
