// https://leetcode.com/problems/top-k-frequent-elements/
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // O(n) time and space
        let n = nums.len();
        // num -> frequency
        nums.into_iter()
            .fold(HashMap::new(), |mut h, a| {
                *h.entry(a).or_insert(0) += 1;
                h
            })
            // frequency -> nums
            .into_iter()
            .fold(vec![vec![]; n + 1], |mut v, (a, freq)| {
                v[n - freq].push(a);
                v
            })
            // 1D array of nums (most frequent first)
            .into_iter()
            .flatten()
            .take(k as usize)
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }
}
