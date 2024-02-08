/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 *
 * https://leetcode.com/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (60.31%)
 * Likes:    30407
 * Dislikes: 460
 * Total Accepted:    1.8M
 * Total Submissions: 3M
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * Given n non-negative integers representing an elevation map where the width
 * of each bar is 1, compute how much water it can trap after raining.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 * Explanation: The above elevation map (black section) is represented by array
 * [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue
 * section) are being trapped.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [4,2,0,3,2,5]
 * Output: 9
 *
 *
 * Constraints:
 *
 *
 *
 * n == height.length
 * 1 <= n <= 2 * 10^4
 * 0 <= height[i] <= 10^5
 *
 *
 */
pub struct Solution;
// @lc code=start
use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_highest, mut right_highest) = (height[left], height[right]);

        while left < right {
            if left_highest < right_highest {
                water += cmp::max(0, left_highest - height[left]);
                left += 1;
                left_highest = cmp::max(left_highest, height[left]);
            } else {
                water += cmp::max(0, right_highest - height[right]);
                right -= 1;
                right_highest = cmp::max(right_highest, height[right]);
            }
        }
        water
    }
}
// @lc code=end

#[test]
fn test_trap() {
    let case1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(Solution::trap(case1), 6);

    let case2 = vec![4, 2, 0, 3, 2, 5];
    assert_eq!(Solution::trap(case2), 9);
}
