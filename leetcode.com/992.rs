// https://leetcode.com/problems/subarrays-with-k-different-integers/
use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let (k, mut imin, mut imax, mut h) = (k as usize, None, None, HashMap::new());
        // let imax be max such that (imax, j) has (exactly) k
        // let imin be min such that (imin+1, j) has (exactly) k
        // fix j, then (i, j) has k iff imin < i <= imax
        nums.iter()
            .map(|&a| /* let a be the jth element */ {
                *h.entry(a).or_insert(0) += 1;
                if h.len() > k {
                    // this iter says (ix, j) has k+1
                    // prev iter says
                    //   (ix, j-1) has k
                    //   (ix+1, j-1) has k-1
                    // thus (ix+1, j) has k
                    let ix = imax.unwrap();
                    assert_eq!(h.remove(&nums[ix]).unwrap(), 1);
                    imin = Some(ix);
                    imax = Some(ix+1);
                }
                if h.len() == k {
                    // find imax
                    let mut ix = imax.unwrap_or(0);
                    while h[&nums[ix]] > 1 {
                        h.entry(nums[ix]).and_modify(|e| *e -= 1);
                        ix += 1;
                    }
                    imax = Some(ix);
                    // (i, j) is valid iff imin < i <= imax
                    imin.map(|im|ix-im).unwrap_or(ix+1)
                } else {
                    0
                }
            })
            .sum::<usize>() as i32
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
    println!("test result: ok.");
}
