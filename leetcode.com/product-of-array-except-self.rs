// https://leetcode.com/problems/product-of-array-except-self/
use std::iter::once;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let prod = |st: &mut i32, &x| {
            *st *= x;
            Some(*st)
        };
        let pref: Vec<_> = nums[..n - 1].iter().scan(1, prod).collect();
        let suf: Vec<_> = nums[1..]
            .iter()
            .rev()
            .scan(1, prod)
            // why is scanning a double ended iter not double ended?
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        once(1)
            .chain(pref.into_iter())
            .zip(suf.into_iter().chain(once(1)))
            .map(|(a, b)| a * b)
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
