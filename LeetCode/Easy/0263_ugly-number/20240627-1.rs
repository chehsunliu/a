impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n == 0 {
            return false;
        }

        let primes = [2, 3, 5];

        for prime in primes {
            while n % prime == 0 {
                n /= prime
            }
        }

        n == 1
    }
}

struct Solution;
