// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        num + num.checked_ilog2().unwrap_or_default() as i32
            - (1..32).map(|i| num >> i).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1480() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
