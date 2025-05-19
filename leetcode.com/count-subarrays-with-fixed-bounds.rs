// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        // count num of subarrays whose min is min_k and max is max_k
        let (mut ibad, mut imin, mut imax) = (-1, -1, -1);
        nums.into_iter()
            .enumerate()
            .map(|(i, x)| (i as i64, x))
            .map(|(i, x)| {
                if x == min_k {
                    imin = i;
                }
                if x == max_k {
                    imax = i;
                }
                if x < min_k || x > max_k {
                    ibad = i;
                }
                0.max(imin.min(imax) - ibad)
            })
            .sum()
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    println!("test result: ok.");
}
