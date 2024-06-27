impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut count = 0;
        let mut v = 0;

        while count < n {
            v += 1;
            if is_ugly(v) {
                count += 1;
            }
        }

        v
    }
}

fn is_ugly(mut n: i32) -> bool {
    let primes = [2, 3, 5];

    for prime in primes {
        while n % prime == 0 {
            n /= prime
        }
    }

    n == 1
}

struct Solution;
