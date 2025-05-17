// https://leetcode.com/problems/count-complete-subarrays-in-an-array/
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len(); // >= 1
        let k = nums.iter().collect::<HashSet<_>>().len();
        // sliding window, O(n) time
        let (mut l, mut r, mut h, mut m) = (0, 0, HashMap::new(), 0);
        // loop invariant:
        // - l <= r
        // - h is freq count on nums[l..r]
        while l < n && r <= n {
            if h.len() < k {
                // not enough distinct elements => try next r
                if let Some(&x) = nums.get(r) {
                    *h.entry(x).or_insert(0) += 1;
                }
                r += 1;
            } else {
                // [l..r], [l..r+1], ..., [l..n] are all valid
                m += n - r + 1;
                // try next l
                h.entry(nums[l]).and_modify(|e| *e -= 1);
                if h[&nums[l]] == 0 {
                    h.remove(&nums[l]);
                }
                l += 1;
            }
        }
        m as i32
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
    assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5, 5]), 10);
    println!("test result: ok.");
}
