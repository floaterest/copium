// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let s: i32 = (1..32).map(|i| num >> i).sum();
        num.ilog2() as i32 + num - s
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
