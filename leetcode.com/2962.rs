// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
use std::collections::VecDeque;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let m = nums.iter().max().unwrap();
        let mut ix = VecDeque::new();
        nums.iter()
            .enumerate()
            .map(|(j, x)| {
                if x == m {
                    ix.push_front(j);
                }
                if ix.len() > k as usize {
                    ix.pop_back();
                }
                if ix.len() == k as usize { *ix.back().unwrap() + 1 } else { 0 }
            })
            .sum::<usize>() as i64
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
    assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
    println!("test result: ok.");
}
