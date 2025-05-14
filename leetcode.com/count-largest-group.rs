// https://leetcode.com/problems/count-largest-group/
pub struct Solution;

use std::{collections::HashMap, iter::from_fn};
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut h: HashMap<i32, i32> = HashMap::new();
        for mut i in 1..n + 1 {
            // sum of digits
            let s = from_fn(move || {
                let u = if i == 0 { None } else { Some(i % 10) };
                i /= 10;
                u
            })
            .sum();
            *h.entry(s).or_default() += 1;
        }
        let max = *h.values().max().unwrap_or(&0);
        h.values().filter(|&&c| c == max).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        assert_eq!(Solution::count_largest_group(13), 4);
        assert_eq!(Solution::count_largest_group(2), 2);
    }
}
