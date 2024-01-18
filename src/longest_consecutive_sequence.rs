/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 *
 * https://leetcode.com/problems/longest-consecutive-sequence/description/
 *
 * algorithms
 * Medium (47.40%)
 * Likes:    19031
 * Dislikes: 895
 * Total Accepted:    1.5M
 * Total Submissions: 3.2M
 * Testcase Example:  '[100,4,200,1,3,2]'
 *
 * Given an unsorted array of integers nums, return the length of the longest
 * consecutive elements sequence.
 *
 * You must write an algorithm that runs in O(n) time.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [100,4,200,1,3,2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4].
 * Therefore its length is 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,3,7,2,5,8,4,6,0,1]
 * Output: 9
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

pub struct Solution {}

// @lc code=start
use std::iter::FromIterator;
use std::{cmp, collections::HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = HashSet::from_iter(nums.into_iter());

        let mut longest_consecutive_len = 0;
        for num in &nums {
            if !nums.contains(&(num - 1)) {
                let mut consecutive_len = 1;
                let mut i = 1;
                loop {
                    if nums.contains(&(num + i)) {
                        consecutive_len += 1;
                        i += 1;
                    } else {
                        break;
                    }
                }
                longest_consecutive_len = cmp::max(longest_consecutive_len, consecutive_len);
            }
        }
        longest_consecutive_len as i32
    }
}
// @lc code=end

#[test]
fn test_longest_consecutive() {
    let case1 = vec![100, 4, 200, 1, 3, 2];
    assert_eq!(Solution::longest_consecutive(case1), 4);

    let case2 = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    assert_eq!(Solution::longest_consecutive(case2), 9);
}
