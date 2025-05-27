// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        for (i, &z) in nums.iter().enumerate().skip(2) {
            let (x, y) = (nums[i - 2], nums[i - 1]);
            if y & 1 == 1 {
                continue;
            }
            if x + z == y / 2 {
                s += 1;
            }
        }
        s
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 4, 1]), 1);
    println!("test result: ok.");
}
