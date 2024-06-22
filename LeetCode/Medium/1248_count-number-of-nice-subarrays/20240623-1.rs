impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut odds: Vec<i32> = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                odds.push(i as i32);
            }
        }

        let mut ans = 0;

        for i in 0..(odds.len() as i32 - k + 1) {
            let odd1 = odds[i as usize];
            let odd2 = odds[(i + k - 1) as usize];

            let odd0 = if i != 0 { odds[i as usize - 1] + 1 } else { 0 };
            let odd3 = if i + k - 1 != odds.len() as i32 - 1 {
                odds[(i + k - 1 + 1) as usize] - 1
            } else {
                nums.len() as i32 - 1
            };

            // [1,1,2,1,1], k=3
            //  0 1 3 4
            ans += (odd1 - odd0 + 1) * (odd3 - odd2 + 1);
        }

        ans
    }
}

struct Solution;
