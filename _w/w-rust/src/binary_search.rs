pub fn search_l<T: PartialOrd>(arr: &[T], x: T) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = arr.len() as i32 - 1;

    while l <= r {
        let m = (l + r) / 2;

        if x <= arr[m as usize] {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

pub fn search_r<T: PartialOrd>(arr: &[T], x: T) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = arr.len() as i32 - 1;

    while l <= r {
        let m = (l + r) / 2;

        if x < arr[m as usize] {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::{search_l, search_r};

    #[test]
    fn search_l_works() {
        let arr = vec![11, 22, 33];
        assert_eq!(search_l(&arr, 0), 0);
        assert_eq!(search_l(&arr, 11), 0);
        assert_eq!(search_l(&arr, 12), 1);
        assert_eq!(search_l(&arr, 22), 1);
        assert_eq!(search_l(&arr, 23), 2);
        assert_eq!(search_l(&arr, 33), 2);
        assert_eq!(search_l(&arr, 44), 3);

        let arr = vec![11, 11, 22, 22, 22, 22, 33, 33];

        assert_eq!(search_l(&arr, 0), 0);
        assert_eq!(search_l(&arr, 11), 0);
        assert_eq!(search_l(&arr, 12), 2);
        assert_eq!(search_l(&arr, 13), 2);
        assert_eq!(search_l(&arr, 22), 2);
        assert_eq!(search_l(&arr, 33), 6);
        assert_eq!(search_l(&arr, 44), 8);
    }

    #[test]
    fn search_r_works() {
        let arr = vec![11, 22, 33];
        assert_eq!(search_r(&arr, 0), 0);
        assert_eq!(search_r(&arr, 11), 1);
        assert_eq!(search_r(&arr, 12), 1);
        assert_eq!(search_r(&arr, 22), 2);
        assert_eq!(search_r(&arr, 23), 2);
        assert_eq!(search_r(&arr, 33), 3);
        assert_eq!(search_r(&arr, 44), 3);

        let arr = vec![11, 11, 22, 22, 22, 22, 33, 33];

        assert_eq!(search_r(&arr, 0), 0);
        assert_eq!(search_r(&arr, 11), 2);
        assert_eq!(search_r(&arr, 12), 2);
        assert_eq!(search_r(&arr, 13), 2);
        assert_eq!(search_r(&arr, 22), 6);
        assert_eq!(search_r(&arr, 33), 8);
        assert_eq!(search_r(&arr, 44), 8);
    }
}
