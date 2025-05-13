// https://leetcode.com/problems/house-robber/
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // assume items are numbered 1, 2, ...
        // dp[i] is best rob for items 1..i

        // dp[i] is either
        //   dp[i - 2] + v_i if take the ith item
        //   dp[i - 1]       if skip the ith item
        let mut dp = Vec::with_capacity(n);
        dp.push(0);
        dp.push(0);
        let mut u = 0;
        for v in nums.iter() {
            let i = dp.len();
            u = dp[i - 1].max(dp[i - 2] + v);
            dp.push(u);
        }
        u
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
