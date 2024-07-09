use std::cmp::max;

struct Customer {
    arrival: i64,
    time: i64,
}

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let customers = customers
            .into_iter()
            .map(|c| Customer {
                arrival: c[0] as i64,
                time: c[1] as i64,
            })
            .collect::<Vec<Customer>>();

        let mut total_waiting_time = 0;
        let mut t = 0;

        for c in &customers {
            t = max(t, c.arrival);
            t += c.time;
            total_waiting_time += t - c.arrival;
        }

        total_waiting_time as f64 / customers.len() as f64
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
