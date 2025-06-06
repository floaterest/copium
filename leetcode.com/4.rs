// https://leetcode.com/problems/median-of-two-sorted-arrays/
impl Solution {
    fn f(ns1: &[i32], ns2: &[i32], i: usize, j: usize) -> f64 {
        // i < j trust me
        let (n1, n2) = (ns1.len(), ns2.len());
        // try cut at nums1 (with its corresponding cut at nums2)
        let m1 = (i + j) / 2;
        let m2 = n1 + n2 - m1;
        // compute endpoints
        let l1 = m1.checked_sub(1).and_then(|i| ns1.get(i / 2)).copied().unwrap_or(i32::MIN);
        let l2 = m2.checked_sub(1).and_then(|i| ns2.get(i / 2)).copied().unwrap_or(i32::MIN);
        let r1 = ns1.get(m1 / 2).copied().unwrap_or(i32::MAX);
        let r2 = ns2.get(m2 / 2).copied().unwrap_or(i32::MAX);
        // binary search
        if l1 > r2 {
            Self::f(ns1, ns2, i, m1 - 1)
        } else if l2 > r1 {
            Self::f(ns1, ns2, m1 + 1, j)
        } else {
            (l1.max(l2) + r1.min(r2)) as f64 / 2.0
        }
    }
    pub fn find_median_sorted_arrays(ns1: Vec<i32>, ns2: Vec<i32>) -> f64 {
        let (n1, n2) = (ns1.len(), ns2.len());
        if n1 <= n2 {
            Self::f(&ns1, &ns2, 0, n1 * 2)
        } else {
            Self::find_median_sorted_arrays(ns2, ns1)
        }
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    println!("test result: ok.");
}
