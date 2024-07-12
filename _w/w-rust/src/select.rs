use rand::random;

fn select_kth_smallest_recursive(nums: &[i32], k: usize) -> i32 {
    // ... < m <= ...
    let m = random::<usize>() % nums.len();

    let mut nums_l: Vec<i32> = vec![];
    let mut nums_r: Vec<i32> = vec![];

    for i in 0..nums.len() {
        if i == m {
            continue;
        }

        if nums[i] >= nums[m] {
            nums_r.push(nums[i]);
        } else {
            nums_l.push(nums[i]);
        }
    }

    if k == nums_l.len() {
        nums[m]
    } else if k < nums_l.len() {
        select_kth_smallest_recursive(&nums_l, k)
    } else {
        select_kth_smallest_recursive(&nums_r, k - nums_l.len() - 1)
    }
}

fn select_kth_smallest(mut nums: Vec<i32>, mut k: usize) -> i32 {
    // ... < m <= ...
    loop {
        let m = random::<usize>() % nums.len();

        let mut nums_l: Vec<i32> = vec![];
        let mut nums_r: Vec<i32> = vec![];

        for i in 0..nums.len() {
            if i == m {
                continue;
            }

            if nums[i] >= nums[m] {
                nums_r.push(nums[i]);
            } else {
                nums_l.push(nums[i]);
            }
        }

        if k == nums_l.len() {
            return nums[m];
        } else if k < nums_l.len() {
            nums = nums_l;
        } else {
            nums = nums_r;
            k = k - nums_l.len() - 1;
        }
    }
}

// 12 5 16 1 7 55 27 (4)
// 5 1 7 | 12 | 16 55 27

// 12 5 16 1 7 55 27
// 5 12 16 1 7 55 27
// 5 12 27 1 7 55 16
//    ^         ^

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_kth_smallest_works() {
        assert_eq!(select_kth_smallest(vec![5], 0), 5);

        assert_eq!(select_kth_smallest(vec![12, 5], 0), 5);
        assert_eq!(select_kth_smallest(vec![12, 5], 1), 12);

        let v3 = vec![12, 5, 16, 1, 7, 55, 27];
        assert_eq!(select_kth_smallest(v3.clone(), 0), 1);
        assert_eq!(select_kth_smallest(v3.clone(), 1), 5);
        assert_eq!(select_kth_smallest(v3.clone(), 2), 7);
        assert_eq!(select_kth_smallest(v3.clone(), 3), 12);
        assert_eq!(select_kth_smallest(v3.clone(), 4), 16);
        assert_eq!(select_kth_smallest(v3.clone(), 5), 27);
        assert_eq!(select_kth_smallest(v3.clone(), 6), 55);
    }

    #[test]
    fn select_kth_smallest_recursive_works() {
        assert_eq!(select_kth_smallest_recursive(&vec![5], 0), 5);

        assert_eq!(select_kth_smallest_recursive(&vec![12, 5], 0), 5);
        assert_eq!(select_kth_smallest_recursive(&vec![12, 5], 1), 12);

        let v3 = vec![12, 5, 16, 1, 7, 55, 27];
        assert_eq!(select_kth_smallest_recursive(&v3, 0), 1);
        assert_eq!(select_kth_smallest_recursive(&v3, 1), 5);
        assert_eq!(select_kth_smallest_recursive(&v3, 2), 7);
        assert_eq!(select_kth_smallest_recursive(&v3, 3), 12);
        assert_eq!(select_kth_smallest_recursive(&v3, 4), 16);
        assert_eq!(select_kth_smallest_recursive(&v3, 5), 27);
        assert_eq!(select_kth_smallest_recursive(&v3, 6), 55);
    }
}
