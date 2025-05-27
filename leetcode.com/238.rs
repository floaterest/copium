// https://leetcode.com/problems/product-of-array-except-self/
use std::iter::once;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let prod = |st: &mut i32, &x| {
            *st *= x;
            Some(*st)
        };
        let pref = once(1).chain(nums[..n - 1].iter().scan(1, prod));
        let suff = nums[1..]
            .iter()
            .rev()
            // why is scan on DoubleEndedIter not DoubleEndedIter?
            .scan(1,prod).collect::<Vec<_>>().into_iter()
            .rev()
            .chain(once(1));
        suff.zip(pref).map(|(a, b)| a * b).collect()
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
