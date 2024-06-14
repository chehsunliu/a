impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();

        let mut i_a = a.len() as i32 - 1;
        let mut i_b = b.len() as i32 - 1;

        let mut ans = vec![];
        let mut carry = 0;

        while i_a >= 0 || i_b >= 0 {
            let bit_a = if i_a >= 0 {
                a[i_a as usize] - '0' as u8
            } else {
                0
            };
            let bit_b = if i_b >= 0 {
                b[i_b as usize] - '0' as u8
            } else {
                0
            };

            let v = bit_a + bit_b + carry;
            ans.push((v % 2 + '0' as u8) as char);
            carry = v / 2;

            i_a -= 1;
            i_b -= 1;
        }

        if carry != 0 {
            ans.push('1');
        }

        ans.reverse();
        String::from_iter(ans)
    }
}

struct Solution;
