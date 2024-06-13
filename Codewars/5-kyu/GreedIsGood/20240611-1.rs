fn score(dice: [u8; 5]) -> u32 {
    let mut counts = [0; 6];
    for d in &dice {
        counts[*d as usize - 1] += 1;
    }

    let mut score: u32 = 0;

    for (i, count) in counts.iter().enumerate() {
        let mut count = *count;
        if i + 1 == 1 {
            if count >= 3 {
                score += 1000;
                count -= 3;
            }

            score += 100 * count;
        } else if i + 1 == 5 {
            if count >= 3 {
                score += 500;
                count -= 3;
            }

            score += 50 * count;
        } else {
            if count >= 3 {
                score += (i as u32 + 1) * 100;
            }
        }
    }

    score
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::score;

    fn dotest(dice: [u8; 5], expected: u32) {
        let actual = score(dice);
        assert!(
            actual == expected,
            "Expected score with dice {dice:?} to be {expected}, but was {actual}\n"
        );
    }

    #[test]
    fn sample_tests() {
        dotest([2, 3, 4, 6, 2], 0);
        dotest([4, 4, 4, 3, 3], 400);
        dotest([2, 4, 4, 5, 4], 450);
    }
}
