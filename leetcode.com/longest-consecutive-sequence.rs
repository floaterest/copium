// https://leetcode.com/problems/longest-consecutive-sequence/
use std::collections::{HashMap, HashSet};
impl Solution {
    fn dfs(s: &i32, edges: &HashMap<i32, Vec<i32>>, vis: &mut HashSet<i32>) {
        if vis.contains(s) {
            return;
        }
        vis.insert(*s);
        for v in &edges[s] {
            Self::dfs(v, edges, vis);
        }
    }
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // O(n) time
        // build graph
        let mut nodes: HashSet<_> = nums.into_iter().map(|n| n * 2).collect();
        let edges: HashMap<i32, Vec<i32>> = nodes
            .iter()
            .flat_map(|&a| [(a - 1, a), (a, a + 1)])
            .fold(HashMap::new(), |mut h, (u, v)| {
                h.entry(u).or_insert(vec![]).push(v);
                h.entry(v).or_insert(vec![]).push(u);
                h
            });
        // find SCCs (consumes nodes)
        let mut max = 0;
        while !nodes.is_empty() {
            let s = nodes.iter().next().unwrap().clone();
            let mut vis = HashSet::new();
            Self::dfs(&s, &edges, &mut vis);
            for v in vis.iter() {
                nodes.remove(v);
            }
            max = vis.iter().filter(|&v| v % 2 == 0).count().max(max);
        }
        max as i32
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    assert_eq!(Solution::longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]), 7);
    println!("test result: ok.");
}
