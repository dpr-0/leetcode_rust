/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 *
 * https://leetcode.com/problems/product-of-array-except-self/description/
 *
 * algorithms
 * Medium (65.13%)
 * Likes:    21006
 * Dislikes: 1221
 * Total Accepted:    2.1M
 * Total Submissions: 3.3M
 * Testcase Example:  '[1,2,3,4]'
 *
 * Given an integer array nums, return an array answer such that answer[i] is
 * equal to the product of all the elements of nums except nums[i].
 *
 * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
 * integer.
 *
 * You must write an algorithm that runs in O(n) time and without using the
 * division operation.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3,4]
 * Output: [24,12,8,6]
 * Example 2:
 * Input: nums = [-1,1,0,-3,3]
 * Output: [0,0,9,0,0]
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^5
 * -30 <= nums[i] <= 30
 * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
 * integer.
 *
 *
 *
 * Follow up: Can you solve the problem in O(1) extra space complexity? (The
 * output array does not count as extra space for space complexity analysis.)
 *
 */

use crate::Solution;

// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1_i32; nums.len()];
        let mut temp_prod = 1;

        // res [1, 1, 1, 1] to [1, 1, 2, 6]
        for i in 0..nums.len() {
            res[i] *= temp_prod;
            temp_prod *= nums[i];
        }
        temp_prod = 1;
        // res [1, 1, 2, 6] to [24, 12, 8, 6]
        for i in (0..nums.len()).rev() {
            res[i] *= temp_prod;
            temp_prod *= nums[i];
        }
        res
    }
}
// @lc code=end

#[test]
fn test_product_except_self() {
    let case1 = vec![1, 2, 3, 4];
    assert_eq!(Solution::product_except_self(case1), [24, 12, 8, 6]);

    let case2 = vec![-1, 1, 0, -3, 3];
    assert_eq!(Solution::product_except_self(case2), [0, 0, 9, 0, 0]);
}
