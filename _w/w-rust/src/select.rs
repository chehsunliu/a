use rand::random;

pub fn select_kth_smallest_recursive(nums: &[i32], k: usize) -> i32 {
    // ... < m <= ...
    let m = random::<usize>() % nums.len();

    let mut nums_l: Vec<i32> = vec![];
    let mut nums_r: Vec<i32> = vec![];
    let mut m_count = 0;

    for i in 0..nums.len() {
        if nums[i] > nums[m] {
            nums_r.push(nums[i]);
        } else if nums[i] < nums[m] {
            nums_l.push(nums[i]);
        } else {
            m_count += 1;
        }
    }

    // 3 4 2
    // 0 1 2 | 3 4 5 6 | 7 8
    if k < nums_l.len() {
        select_kth_smallest_recursive(&nums_l, k)
    } else if k >= nums_l.len() + m_count {
        select_kth_smallest_recursive(&nums_r, k - nums_l.len() - m_count)
    } else {
        nums[m]
    }
}

pub fn select_kth_smallest(mut nums: Vec<i32>, mut k: usize) -> i32 {
    // ... < m <= ...
    loop {
        let m = random::<usize>() % nums.len();

        let mut nums_l: Vec<i32> = vec![];
        let mut nums_r: Vec<i32> = vec![];
        let mut m_count = 0;

        for i in 0..nums.len() {
            if nums[i] > nums[m] {
                nums_r.push(nums[i]);
            } else if nums[i] < nums[m] {
                nums_l.push(nums[i]);
            } else {
                m_count += 1;
            }
        }

        if k < nums_l.len() {
            nums = nums_l;
        } else if k >= nums_l.len() + m_count {
            nums = nums_r;
            k = k - nums_l.len() - m_count;
        } else {
            return nums[m];
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

        let v = vec![12, 5, 16, 1, 7, 55, 27];
        assert_eq!(select_kth_smallest(v.clone(), 0), 1);
        assert_eq!(select_kth_smallest(v.clone(), 1), 5);
        assert_eq!(select_kth_smallest(v.clone(), 2), 7);
        assert_eq!(select_kth_smallest(v.clone(), 3), 12);
        assert_eq!(select_kth_smallest(v.clone(), 4), 16);
        assert_eq!(select_kth_smallest(v.clone(), 5), 27);
        assert_eq!(select_kth_smallest(v.clone(), 6), 55);

        let v = vec![3, 2, 4, 1, 1, 2, 2, 2, 2, 2, 5];
        assert_eq!(select_kth_smallest(v.clone(), 0), 1);
        assert_eq!(select_kth_smallest(v.clone(), 1), 1);
        assert_eq!(select_kth_smallest(v.clone(), 2), 2);
        assert_eq!(select_kth_smallest(v.clone(), 3), 2);
        assert_eq!(select_kth_smallest(v.clone(), 4), 2);
        assert_eq!(select_kth_smallest(v.clone(), 5), 2);
        assert_eq!(select_kth_smallest(v.clone(), 6), 2);
        assert_eq!(select_kth_smallest(v.clone(), 7), 2);
        assert_eq!(select_kth_smallest(v.clone(), 8), 3);
        assert_eq!(select_kth_smallest(v.clone(), 9), 4);
        assert_eq!(select_kth_smallest(v.clone(), 10), 5);
    }

    #[test]
    fn select_kth_smallest_recursive_works() {
        assert_eq!(select_kth_smallest_recursive(&vec![5], 0), 5);

        assert_eq!(select_kth_smallest_recursive(&vec![12, 5], 0), 5);
        assert_eq!(select_kth_smallest_recursive(&vec![12, 5], 1), 12);

        let v = vec![12, 5, 16, 1, 7, 55, 27];
        assert_eq!(select_kth_smallest_recursive(&v, 0), 1);
        assert_eq!(select_kth_smallest_recursive(&v, 1), 5);
        assert_eq!(select_kth_smallest_recursive(&v, 2), 7);
        assert_eq!(select_kth_smallest_recursive(&v, 3), 12);
        assert_eq!(select_kth_smallest_recursive(&v, 4), 16);
        assert_eq!(select_kth_smallest_recursive(&v, 5), 27);
        assert_eq!(select_kth_smallest_recursive(&v, 6), 55);

        let v = vec![3, 2, 4, 1, 1, 2, 2, 2, 2, 2, 5];
        assert_eq!(select_kth_smallest_recursive(&v, 0), 1);
        assert_eq!(select_kth_smallest_recursive(&v, 1), 1);
        assert_eq!(select_kth_smallest_recursive(&v, 2), 2);
        assert_eq!(select_kth_smallest_recursive(&v, 3), 2);
        assert_eq!(select_kth_smallest_recursive(&v, 4), 2);
        assert_eq!(select_kth_smallest_recursive(&v, 5), 2);
        assert_eq!(select_kth_smallest_recursive(&v, 6), 2);
        assert_eq!(select_kth_smallest_recursive(&v, 7), 2);
        assert_eq!(select_kth_smallest_recursive(&v, 8), 3);
        assert_eq!(select_kth_smallest_recursive(&v, 9), 4);
        assert_eq!(select_kth_smallest_recursive(&v, 10), 5);
    }
}
