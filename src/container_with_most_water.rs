/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 *
 * https://leetcode.com/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (54.47%)
 * Likes:    27947
 * Dislikes: 1609
 * Total Accepted:    2.7M
 * Total Submissions: 4.9M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * You are given an integer array height of length n. There are n vertical
 * lines drawn such that the two endpoints of the i^th line are (i, 0) and (i,
 * height[i]).
 *
 * Find two lines that together with the x-axis form a container, such that the
 * container contains the most water.
 *
 * Return the maximum amount of water a container can store.
 *
 * Notice that you may not slant the container.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array
 * [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the
 * container can contain is 49.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */
pub struct Solution {}
// @lc code=start
use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        while left < right {
            let new_area = cmp::min(height[left], height[right]) * ((right - left) as i32);
            area = cmp::max(new_area, area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        area
    }
}
// @lc code=end

#[test]
fn test_max_area() {
    let case1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(case1), 49);

    let case2 = vec![1, 1];
    assert_eq!(Solution::max_area(case2), 1);
}
