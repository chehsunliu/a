impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats.clone();
        let mut students = students.clone();

        seats.sort();
        students.sort();

        let mut ans = 0;

        for i in 0..seats.len() {
            let mut diff = seats[i] - students[i];
            if diff < 0 {
                diff = -diff;
            }

            ans += diff;
        }

        ans
    }
}

struct Solution;
