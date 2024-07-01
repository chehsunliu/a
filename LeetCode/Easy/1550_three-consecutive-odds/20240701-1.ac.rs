impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for i in 0..(arr.len() as i32 - 2) {
            if arr[i as usize] % 2 == 1
                && arr[i as usize + 1] % 2 == 1
                && arr[i as usize + 2] % 2 == 1
            {
                return true;
            }
        }

        false
    }
}

struct Solution;
