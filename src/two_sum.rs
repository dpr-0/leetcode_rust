/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
use crate::Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table: HashMap<i32, usize> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if table.contains_key(n) {
                return Vec::from([table[n] as i32, i as i32]);
            } else {
                table.insert(target - *n, i);
            }
        }
        Vec::from([0, 0])
    }
}
// @lc code=end

#[test]
fn test_two_sum() {
    let case1 = vec![2, 7, 11, 15];
    assert_eq!(Solution::two_sum(case1, 9), [0, 1]);
}
