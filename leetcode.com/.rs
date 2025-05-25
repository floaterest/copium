// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|x| x.ilog10()).filter(|n| n & 1 == 1).count() as i32
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    println!("test result: ok.");
}
