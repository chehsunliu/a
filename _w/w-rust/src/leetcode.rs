impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        heap_sort(&mut nums);
        nums
    }
}

fn max_heapify(nums: &mut Vec<i32>, i: usize, heap_size: usize) {
    let (l, r) = (i * 2 + 1, i * 2 + 2);
    let largest_i = if l < heap_size && nums[l] > nums[i] {
        l
    } else {
        i
    };
    let largest_i = if r < heap_size && nums[r] > nums[largest_i] {
        r
    } else {
        largest_i
    };

    if i != largest_i {
        nums.swap(i, largest_i);
        max_heapify(nums, largest_i, heap_size);
    }
}

fn build_max_heap(nums: &mut Vec<i32>) {
    for i in (0..nums.len() / 2).rev() {
        max_heapify(nums, i, nums.len());
    }
}

fn heap_sort(nums: &mut Vec<i32>) {
    build_max_heap(nums);

    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        max_heapify(nums, 0, i);
    }
}

//    0
//  1   2
// 3 4 5 6

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
