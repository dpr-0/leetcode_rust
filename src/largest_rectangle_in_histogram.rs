/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 *
 * https://leetcode.com/problems/largest-rectangle-in-histogram/description/
 *
 * algorithms
 * Hard (43.70%)
 * Likes:    16604
 * Dislikes: 251
 * Total Accepted:    828.5K
 * Total Submissions: 1.9M
 * Testcase Example:  '[2,1,5,6,2,3]'
 *
 * Given an array of integers heights representing the histogram's bar height
 * where the width of each bar is 1, return the area of the largest rectangle
 * in the histogram.
 *
 *
 * Example 1:
 *
 *
 * Input: heights = [2,1,5,6,2,3]
 * Output: 10
 * Explanation: The above is a histogram where width of each bar is 1.
 * The largest rectangle is shown in the red area, which has an area = 10
 * units.
 *
 *
 * Example 2:
 *
 *
 * Input: heights = [2,4]
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= heights.length <= 10^5
 * 0 <= heights[i] <= 10^4
 *
 *
 */
pub struct Solution;
// @lc code=start
use std::cmp;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new();
        for (i2, &h) in heights.iter().enumerate() {
            let mut start = i2;

            while !stack.is_empty() && h < stack.last().unwrap().1 {
                let (i1, h) = stack.pop().unwrap();
                max_area = cmp::max(max_area, h * (i2 - i1) as i32);
                start = i1;
            }
            stack.push((start, h));
        }
        for (i, h) in stack {
            max_area = cmp::max(max_area, h * (heights.len() - i) as i32);
        }
        max_area
    }
}
// @lc code=end
#[test]
fn test_largest_rectangle_area() {
    let case1 = vec![2, 1, 5, 6, 2, 3];
    assert_eq!(Solution::largest_rectangle_area(case1), 10);

    let case2 = vec![2, 4];
    assert_eq!(Solution::largest_rectangle_area(case2), 4);
}
