/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 *
 * https://leetcode.com/problems/3sum/description/
 *
 * algorithms
 * Medium (33.77%)
 * Likes:    29674
 * Dislikes: 2709
 * Total Accepted:    3.2M
 * Total Submissions: 9.5M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j],
 * nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] +
 * nums[k] == 0.
 *
 * Notice that the solution set must not contain duplicate triplets.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Explanation:
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
 * The distinct triplets are [-1,0,1] and [-1,-1,2].
 * Notice that the order of the output and the order of the triplets does not
 * matter.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,1,1]
 * Output: []
 * Explanation: The only possible triplet does not sum up to 0.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [0,0,0]
 * Output: [[0,0,0]]
 * Explanation: The only possible triplet sums up to 0.
 *
 *
 *
 * Constraints:
 *
 *
 * 3 <= nums.length <= 3000
 * -10^5 <= nums[i] <= 10^5
 *
 *
 */
pub struct Solution;
// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        for i in 0..(nums.len()) {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let three_sum = nums[i] + nums[left] + nums[right];
                match three_sum.cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        while nums[left] == nums[left - 1] && left < right {
                            left += 1;
                        }
                    }
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                }
            }
        }
        res
    }
}
// @lc code=end

#[test]
fn test_three_sum() {
    let case1 = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(
        Solution::three_sum(case1),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    )
}
