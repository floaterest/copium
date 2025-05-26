// https://leetcode.com/problems/container-with-most-water/
impl Solution {
    fn f(xs: &[i32]) -> i32 {
        if xs.len() <= 1 {
            return 0;
        }
        let (a, b) = (xs.first().unwrap(), xs.last().unwrap());
        if a <= b {
            Self::f(&xs[1..])
        } else {
            Self::f(&xs[..xs.len() - 1])
        }
        .max((xs.len() as i32 - 1) * a.min(b))
    }
    pub fn max_area(xs: Vec<i32>) -> i32 {
        Self::f(&xs)
    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    println!("test result: ok.");
}
