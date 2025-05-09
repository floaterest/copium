// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let s = format!("{num:b}");
        (s.len() + s.chars().filter(|&c| c == '1').count()) as i32 - 1
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
