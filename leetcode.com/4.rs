// https://leetcode.com/problems/median-of-two-sorted-arrays/
impl Solution {
    pub fn find_median_sorted_arrays(ns1: Vec<i32>, ns2: Vec<i32>) -> f64 {
        let (n1, n2) = (ns1.len(), ns2.len());
        if n1 < n2 {
            return Self::find_median_sorted_arrays(ns2, ns1);
        }
        // binary search on 2m+1
        let (mut i, mut j) = (0, n2 * 2);
        while i <= j {
            // try cut at B (with its corresponding cut at A)
            let m2 = (i + j) / 2;
            let m1 = n1 + n2 - m2;
            // compute endpoints
            let l1 = m1.checked_sub(1).and_then(|i| ns1.get(i / 2).copied()).unwrap_or(i32::MIN);
            let l2 = m2.checked_sub(1).and_then(|i| ns2.get(i / 2).copied()).unwrap_or(i32::MIN);
            let r1 = ns1.get(m1 / 2).copied().unwrap_or(i32::MAX);
            let r2 = ns2.get(m2 / 2).copied().unwrap_or(i32::MAX);
            // binary search
            if l1 > r2 {
                i = m2 + 1;
            } else if l2 > r1 {
                j = m2 - 1;
            } else {
                return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
            }
        }
        -1.0
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    println!("test result: ok.");
}
