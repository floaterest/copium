// https://leetcode.com/problems/count-subarrays-with-score-less-than-k/
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let nums: Vec<_> = nums.into_iter().map(|x| x as i64).collect();
        let mut ix: Option<usize> = None;
        let (mut s, mut l) = (0, 0);
        // add a => (s+a)*(l+1)
        // remove a => (s-a) * (l-1)
        nums.iter()
            .enumerate()
            .map(|(j, x)| {
                // update score at x
                s += x;
                l += 1;
                // ix == None ==> s*l is score of (0, j)
                ix = ix.or((s * l >= k).then(|| 0));
                // let i be max such that score of (i,j) is >= k
                while let Some(i) = ix {
                    assert!(l > 0);
                    if (s - nums[i]) * (l - 1) < k {
                        break;
                    }
                    s -= nums[i];
                    l -= 1;
                    ix = Some(i + 1);
                }
                assert!(ix.is_none_or(|_| s * l >= k));
                // println!("{ix:?} {j} => {}", s * l);
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
