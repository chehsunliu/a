/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n as i64;

        while l <= r {
            let m = (l + r) / 2;
            let g = guess(m as i32);

            if g == 0 {
                return m as i32;
            } else if g > 0 {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        -1
    }
}

unsafe fn guess(num: i32) -> i32 {
    0
}

struct Solution;
