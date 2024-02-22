/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 *
 * https://leetcode.com/problems/binary-search/description/
 *
 * algorithms
 * Easy (56.89%)
 * Likes:    11444
 * Dislikes: 234
 * Total Accepted:    2.2M
 * Total Submissions: 3.9M
 * Testcase Example:  '[-1,0,3,5,9,12]\n9'
 *
 * Given an array of integers nums which is sorted in ascending order, and an
 * integer target, write a function to search target in nums. If target exists,
 * then return its index. Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 < nums[i], target < 10^4
 * All the integers in nums are unique.
 * nums is sorted in ascending order.
 *
 *
 */
pub struct Solution;
// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, (nums.len() - 1) as i32);
        let res = -1;
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        res
    }
}
// @lc code=end

#[test]
fn test_search() {
    let (case, target) = (vec![-1, 0, 3, 5, 9, 12], 9);
    assert_eq!(Solution::search(case, target), 4);

    let (case, target) = (vec![-1, 0, 3, 5, 9, 12], 2);
    assert_eq!(Solution::search(case, target), -1);

    let (case, target) = (vec![5], -5);
    assert_eq!(Solution::search(case, target), -1);
}
