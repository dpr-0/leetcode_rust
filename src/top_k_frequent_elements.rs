/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

use crate::Solution;

// @lc code=start
use std::collections::HashMap;
use std::iter::FromIterator;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut table: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            table.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        let mut res = Vec::from_iter(table);

        res.sort_by(|a, b| b.1.cmp(&a.1));
        res.iter().map(|tuple| tuple.0).take(k as usize).collect()
    }
}
// @lc code=end

#[test]
fn test_top_k_frequent() {
    let case1 = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(Solution::top_k_frequent(case1, 2), [1, 2]);

    let case2 = vec![1];
    assert_eq!(Solution::top_k_frequent(case2, 1), [1]);

    let case3 = vec![4, 1, -1, 2, -1, 2, 3];
    assert_eq!(Solution::top_k_frequent(case3, 2), [2, -1]);
}
