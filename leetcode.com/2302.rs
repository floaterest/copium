// https://leetcode.com/problems/count-subarrays-with-score-less-than-k/
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let (mut ix, mut s, mut l) = (None, 0, 0);
        let nums: Vec<i64> = nums.into_iter().map(i32::into).collect();
        nums.iter()
            .enumerate()
            .map(|(j, &x)| {
                // update score at x
                (s, l) = (s + x, l + 1);
                // ix == None ==> s*l is score of (0, j)
                ix = ix.or((s * l >= k).then(|| 0));
                // let i be max such that score of (i,j) is >= k
                while let Some((i, y)) = ix.map(|i| (i, nums[i])) {
                    assert!(l > 0);
                    // if remove y makes score less than k
                    if (s - y) * (l - 1) < k {
                        break;
                    }
                    (s, l) = (s - y, l - 1);
                    ix = Some(i + 1);
                }
                assert!(ix.is_none_or(|_| s * l >= k));
                // let cj be count of valid (i, j) for i <= j
                // if no such i exists
                //    then only (i', j) valid for any 0 <= i' <= j
                //    thus cj = j + 1
                // otherwise
                //    only (i',j) valid for any i < i' <= j
                //    thus cj = j - i
                ix.map(|i| j - i).unwrap_or(j + 1)
            })
            .sum::<usize>() as i64
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 5), 5);
    assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
    println!("test result: ok.");
}
