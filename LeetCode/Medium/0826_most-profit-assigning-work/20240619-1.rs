use std::cmp::max;
use std::iter::zip;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs = zip(difficulty, profit)
            .map(|(d, p)| Job {
                difficulty: d,
                profit: p,
            })
            .collect::<Vec<Job>>();

        jobs.sort_by_key(|job| job.difficulty);

        let mut profit = 0;
        for job in jobs.iter_mut() {
            profit = max(profit, job.profit);
            job.profit = profit;
        }

        let mut max_profit = 0;
        for w in &worker {
            max_profit += get_max_profit(&jobs, *w);
        }

        max_profit
    }
}

struct Job {
    difficulty: i32,
    profit: i32,
}

fn get_max_profit(jobs: &[Job], max_difficulty: i32) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = jobs.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        let job = &jobs[m as usize];

        if max_difficulty < job.difficulty {
            r = m - 1;
        } else if max_difficulty > job.difficulty {
            l = m + 1;
        } else {
            l = m + 1;
        }
    }

    if l == 0 {
        0
    } else {
        jobs[l as usize - 1].profit
    }
}

struct Solution;
