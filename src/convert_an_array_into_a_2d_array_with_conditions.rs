/*
* @lc app=leetcode id=2610 lang=rust
*
* [2610] Convert an Array Into a 2D Array With Conditions
*
* https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/description/
*
* algorithms
* Medium (83.41%)
* Likes:    837
* Dislikes: 35
* Total Accepted:    80.6K
* Total Submissions: 92.5K
* Testcase Example:  '[1,3,4,1,2,3,1]'
*
* You are given an integer array nums. You need to create a 2D array from nums
* satisfying the following conditions:
*
*
* The 2D array should contain only the elements of the array nums.
* Each row in the 2D array contains distinct integers.
* The number of rows in the 2D array should be minimal.
*
*
* Return the resulting array. If there are multiple answers, return any of
* them.
*
* Note that the 2D array can have a different number of elements on each
* row.
*
*
* Example 1:
*
*
* Input: nums = [1,3,4,1,2,3,1]
* Output: [[1,3,4,2],[1,3],[1]]
* Explanation: We can create a 2D array that contains the following rows:
* - 1,3,4,2
* - 1,3
* - 1
* All elements of nums were used, and each row of the 2D array contains
* distinct integers, so it is a valid answer.
* It can be shown that we cannot have less than 3 rows in a valid array.
*
* Example 2:
*
*
* Input: nums = [1,2,3,4]
* Output: [[4,3,2,1]]
* Explanation: All elements of the array are distinct, so we can keep all of
* them in the first row of the 2D array.
*
*
*
* Constraints:
*
*
* 1 <= nums.length <= 200
* 1 <= nums[i] <= nums.length
*
*
*/
pub struct Solution;

// @lc code=start
use std::collections::BTreeSet;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut visited_indexes = BTreeSet::new();
        while visited_indexes.len() != nums.len() {
            let mut elements = BTreeSet::new();
            for (i, num) in nums.iter().enumerate() {
                if visited_indexes.contains(&i) || elements.contains(num) {
                    continue;
                }

                visited_indexes.insert(i);
                elements.insert(*num);
            }
            res.push(elements.clone().into_iter().collect());
        }
        res
    }
}
// @lc code=end

#[test]
fn test_find_matrix() {
    let case1 = vec![1, 3, 4, 1, 2, 3, 1];
    assert_eq!(
        Solution::find_matrix(case1),
        vec![vec![1, 2, 3, 4], vec![1, 3], vec![1]]
    );

    let case2 = vec![1, 2, 3, 4];
    assert_eq!(Solution::find_matrix(case2), vec![vec![1, 2, 3, 4]]);
}
