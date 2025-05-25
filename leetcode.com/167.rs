// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
impl Solution {
    pub fn two_sum(ns: Vec<i32>, t: i32) -> Vec<i32> {
        for (i, &a) in ns.iter().enumerate() {
            if let Ok(j) = ns[i + 1..].binary_search(&(t - a)) {
                return [i + 1, i + j + 2].into_iter().map(|x| x as i32).collect();
            }
        }
        unreachable!()
    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    println!("test result: ok.");
}
