impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut ans_l = vec![];
        let mut ans_r = vec![];

        let m = search_l(&arr, x) as i32;
        let mut l = m - 1;
        let mut r = m;

        for _ in 0..k {
            if l >= 0 && r < arr.len() as i32 {
                let v_l = arr[l as usize];
                let v_r = arr[r as usize];
                if x - v_l < v_r - x {
                    ans_l.push(v_l);
                    l -= 1;
                } else if x - v_l > v_r - x {
                    ans_r.push(v_r);
                    r += 1;
                } else {
                    if v_l < v_r {
                        ans_l.push(v_l);
                        l -= 1;
                    } else {
                        ans_r.push(v_r);
                        r += 1;
                    }
                }
            } else if l >= 0 {
                ans_l.push(arr[l as usize]);
                l -= 1;
            } else if r < arr.len() as i32 {
                ans_r.push(arr[r as usize]);
                r += 1;
            } else {
                panic!("GG");
            }
        }

        let mut ans = Vec::with_capacity(k as usize);

        for v in ans_l.iter().rev() {
            ans.push(*v);
        }
        for v in ans_r.iter() {
            ans.push(*v);
        }

        ans
    }
}

fn search_l(arr: &[i32], x: i32) -> usize {
    let mut l: i32 = 0;
    let mut r = arr.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        if arr[m as usize] < x {
            l += 1;
        } else {
            r -= 1;
        }
    }

    l as usize
}

struct Solution;
