impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut ans = digits.clone();
        ans.reverse();
        ans.push(0);

        let mut carry = 1;
        for i in 0..ans.len() {
            let v = ans[i] + carry;
            ans[i] = v % 10;
            carry = v / 10;
        }

        if *ans.last().unwrap() == 0 {
            ans.pop();
        }
        ans.reverse();

        ans
    }
}

struct Solution;
