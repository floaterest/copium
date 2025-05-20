// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut ibad, mut imin, mut imax) = (None, None, None);
        // ibad is latest index where num is <m and >M
        // imin is latest index where num is =m
        // imax is latest index where num is =M
        // thus (i, j) is valid iff ibad < i <= min(imin, imax)
        nums.into_iter()
            .enumerate()
            .map(|(j, x)| {
                if x == min_k {
                    imin = Some(j);
                }
                if x == max_k {
                    imax = Some(j);
                }
                if x < min_k || x > max_k {
                    ibad = Some(j);
                }
                if let Some(i) = imin.and_then(|i1| imax.map(|i2| i1.min(i2))) {
                    if let Some(ib) = ibad { if ib < i { i - ib } else { 0 } } else { i + 1 }
                } else {
                    0
                }
            })
            .sum::<usize>() as i64
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    println!("test result: ok.");
}
