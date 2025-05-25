// https://leetcode.com/problems/push-dominoes/
impl Solution {
    pub fn push_dominoes(ds: String) -> String {
        let mut ls: Vec<_> = ds
            .chars()
            .rev()
            .scan(0, |i, c| {
                *i = if c == 'L' {
                    1
                } else if *i == 0 || c == 'R' {
                    0
                } else {
                    *i + 1
                };
                Some(*i)
            })
            .collect();
        ls.reverse();
        let mut i = 0;
        let rs: Vec<_> = ds
            .chars()
            .map(|c| {
                i = if c == 'R' {
                    1
                } else if i == 0 || c == 'L' {
                    0
                } else {
                    i + 1
                };
                i
            })
            .collect();
        ls.into_iter().zip(rs.into_iter()).map(|(l, r)| {
            if l == r {
                '.'
            } else if l == 0 {
                'R'
            } else if r == 0 || l < r {
                'L'
            } else {
                'R'
            }
        }).collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::push_dominoes(".L.R...LR..L..".into()), "LL.RR.LLRRLL..");
    println!("test result: ok.");
}
