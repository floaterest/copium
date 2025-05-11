// https://leetcode.com/problems/valid-parentheses/
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => v.push(ch),
                _ => {
                    if let Some(a) = v.pop() {
                        let b = match a {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            _ => unreachable!(),
                        };
                        if b != ch {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        v.is_empty()
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
