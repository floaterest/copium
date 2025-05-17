// https://leetcode.com/problems/subarrays-with-k-different-integers/
use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        // O(n) sliding window
        let k = k as usize;
        nums.iter()
            .scan((0, 0, HashMap::new()), |(i, m, h), a| {
                *h.entry(*a).or_insert(0) += 1;
                if h.len() > k {
                    assert_eq!(h.len(), k + 1);
                    assert_eq!(h.remove(&nums[*m]).unwrap(), 1);
                    *m += 1;
                    *i = *m;
                }
                if h.len() == k {
                    while h[&nums[*m]] > 1 {
                        h.entry(nums[*m]).and_modify(|e| *e -= 1);
                        *m += 1;
                    }
                    Some(*m - *i + 1)
                } else {
                    Some(0)
                }
            })
            .sum::<usize>() as i32
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
    println!("test result: ok.");
}
