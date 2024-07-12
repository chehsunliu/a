use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals
            .iter()
            .map(|interval| (interval[0], interval[1]))
            .collect::<Vec<(i32, i32)>>();
        intervals.sort();

        let mut ans = 0;

        let mut priority_queue: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for interval in intervals {
            while let Some(time_end) = priority_queue.peek() {
                if time_end.0 > interval.0 {
                    break;
                }

                priority_queue.pop();
            }
            priority_queue.push(Reverse(interval.1));
            ans = max(ans, priority_queue.len() as i32);
        }

        ans
    }
}

//  0--------30
//     5-10
//      8-20
//         22-------40

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
