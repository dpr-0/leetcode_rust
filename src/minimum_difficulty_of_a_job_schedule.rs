/*
 * @lc app=leetcode id=1335 lang=rust
 *
 * [1335] Minimum Difficulty of a Job Schedule
 */

pub struct Solution;

// @lc code=start
use std::cmp;
use std::collections::HashMap;

fn min_difficulty(
    table: &mut HashMap<(usize, i32, i32), i32>,
    job_difficulty: &[i32],
    curr: usize,
    d: i32,
    daily_max_difficulty: i32,
) -> i32 {
    if curr == job_difficulty.len() && d == 0 {
        return 0;
    }
    if d == 0 || curr == job_difficulty.len() {
        return i32::MAX;
    }
    if let Some(ans) = table.get(&(curr, d, daily_max_difficulty)) {
        return ans.clone();
    }
    let daily_max_difficulty = cmp::max(daily_max_difficulty, job_difficulty[curr]);
    let ans = cmp::min(
        min_difficulty(table, job_difficulty, curr + 1, d - 1, 0)
            .saturating_add(daily_max_difficulty),
        min_difficulty(table, job_difficulty, curr + 1, d, daily_max_difficulty),
    );
    table.insert((curr, d, daily_max_difficulty), ans);
    ans
}

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        if job_difficulty.len() < d as usize {
            return -1;
        }

        if job_difficulty.len() as i32 == d {
            return job_difficulty.iter().sum();
        }
        let mut table = HashMap::new();
        min_difficulty(&mut table, &job_difficulty[..], 0, d, 0)
    }
}
// @lc code=end

#[test]
fn test_min_difficulty() {
    let case1 = vec![6, 5, 4, 3, 2, 1];
    assert_eq!(Solution::min_difficulty(case1, 2), 7);

    let case2 = vec![9, 9, 9];
    assert_eq!(Solution::min_difficulty(case2, 4), -1);

    let case3 = vec![1, 1, 1];
    assert_eq!(Solution::min_difficulty(case3, 3), 3);

    let case4 = vec![
        380, 302, 102, 681, 863, 676, 243, 671, 651, 612, 162, 561, 394, 856, 601, 30, 6, 257, 921,
        405, 716, 126, 158, 476, 889, 699, 668, 930, 139, 164, 641, 801, 480, 756, 797, 915, 275,
        709, 161, 358, 461, 938, 914, 557, 121, 964, 315,
    ];
    assert_eq!(Solution::min_difficulty(case4, 10), 3807);
}
