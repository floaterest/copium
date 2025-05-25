// https://leetcode.com/problems/push-dominoes/
impl Solution {
    pub fn push_dominoes(ds: String) -> String {
        let mut i = 0;
        let ls: Vec<_> = ds
            .chars()
            .rev()
            .map(|c| {
                i = match (c, i) {
                    ('L', _) => 1,
                    ('R', _) | (_, 0) => 0,
                    _ => i + 1,
                };
                i
            })
            .collect::<Vec<_>>();
        use std::cmp::Ordering::*;
        ds.chars()
            .zip(ls.into_iter().rev())
            .scan(0, |r, (c, l)| {
                *r = match (c, *r) {
                    ('R', _) => 1,
                    ('L', _) | (_, 0) => 0,
                    _ => *r + 1,
                };
                Some(match (l, *r, l.cmp(&r)) {
                    (_, _, Equal) => '.',
                    (0, _, _) => 'R',
                    (_, 0, _) => 'L',
                    (_, _, Less) => 'L',
                    (_, _, Greater) => 'R',
                })
            })
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::push_dominoes(".L.R...LR..L..".into()), "LL.RR.LLRRLL..");
    println!("test result: ok.");
}
