// https://leetcode.com/problems/valid-parentheses/
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        let pair = |a| match a {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => unreachable!(),
        };
        // find the char that makes it an NO instance
        let u = s.chars().find(|&ch| match ch {
            '(' | '[' | '{' => {
                v.push(ch);
                false
            }
            _ => v.pop().map(|a| pair(a) != ch).unwrap_or(true),
        });
        u.is_none() && v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid("([])".into()), true);
        assert_eq!(Solution::is_valid("[{]}".into()), false);
    }
}
