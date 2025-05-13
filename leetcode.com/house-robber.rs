// https://leetcode.com/problems/house-robber/
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // assume items are numbered 1, 2, ...
        // dp[i] is best rob for items 1..i

        // dp[i] is either
        //   dp[i - 2] + v_i if take the ith item
        //   dp[i - 1]       if skip the ith item
        let (mut a1, mut a2) = (0, 0);
        for v in nums.iter() {
            (a2, a1) = (a1, a1.max(a2 + v));
        }
        a1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
