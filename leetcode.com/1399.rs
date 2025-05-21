// https://leetcode.com/problems/count-largest-group/
impl Solution {
    fn sum_digits(n: i32) -> i32 {
        if n < 10 { n } else { n % 10 + Self::sum_digits(n / 10) }
    }
    pub fn count_largest_group(n: i32) -> i32 {
        let n = n + 1;
        let mut v = vec![0; 10 + (n / 10) as usize];
        for k in 1..n {
            v[Self::sum_digits(k) as usize] += 1;
        }
        let m = *v.iter().max().unwrap();
        v.iter().filter(|&&a| a == m).count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        assert_eq!(Solution::count_largest_group(13), 4);
        assert_eq!(Solution::count_largest_group(2), 2);
        assert_eq!(Solution::count_largest_group(174), 1);
    }
}
