// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let a = num.ilog2() as i32;
        let b = format!("{num:b}").chars().filter(|&c| c == '1').count() as i32;
        a + b
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
