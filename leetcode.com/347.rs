// https://leetcode.com/problems/top-k-frequent-elements/
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let h = nums.into_iter().fold(HashMap::new(), |mut h, x| {
            *h.entry(x).or_insert(0) += 1;
            h
        });
        let mut h: BinaryHeap<_> = h.into_iter().map(|(x, c)| (c, x)).collect();
        std::iter::from_fn(|| h.pop())
            .take(k as usize)
            .map(|(_, x)| x)
            .collect()
    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    println!("test result: ok.");
}
