// https://leetcode.com/problems/longest-consecutive-sequence/
use std::{collections::HashSet, iter::successors};
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // O(n) time
        let set: HashSet<_> = nums.into_iter().collect();
        set.iter()
            .filter(|&a| !set.contains(&(a - 1)))
            // count streak
            .map(|&a| successors(Some(a), |&a| Some(a + 1)).take_while(|a| set.contains(a)).count())
            .max()
            .unwrap_or(0) as i32
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    assert_eq!(Solution::longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]), 7);
    println!("test result: ok.");
}
