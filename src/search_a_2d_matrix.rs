/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 *
 * https://leetcode.com/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (49.49%)
 * Likes:    15258
 * Dislikes: 403
 * Total Accepted:    1.7M
 * Total Submissions: 3.3M
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,60]]\n3'
 *
 * You are given an m x n integer matrix matrix with the following two
 * properties:
 *
 *
 * Each row is sorted in non-decreasing order.
 * The first integer of each row is greater than the last integer of the
 * previous row.
 *
 *
 * Given an integer target, return true if target is in matrix or false
 * otherwise.
 *
 * You must write a solution in O(log(m * n)) time complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1 <= m, n <= 100
 * -10^4 <= matrix[i][j], target <= 10^4
 *
 *
 */

pub struct Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut top, mut bot) = (0_i32, (matrix.len() - 1) as i32);
        let mut row = -1;
        while top <= bot {
            let mid = top + (bot - top) / 2;
            if target < matrix[mid as usize][0] {
                bot = mid - 1;
            } else if target > *matrix[mid as usize].last().unwrap() {
                top = mid + 1;
            } else {
                row = mid;
                break;
            }
        }

        if row == -1 {
            return false;
        }

        let nums = &matrix[row as usize];
        let (mut left, mut right) = (0, (nums.len() - 1) as i32);
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }

        false
    }
}
// @lc code=end

#[test]
fn test_search_matrix() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    assert!(Solution::search_matrix(matrix, target));

    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 13;
    assert!(!Solution::search_matrix(matrix, target));

    let matrix = vec![vec![1]];
    let target = 0;
    assert!(!Solution::search_matrix(matrix, target));

    let matrix = vec![vec![1]];
    let target = 1;
    assert!(Solution::search_matrix(matrix, target));

    let matrix = vec![vec![1, 1]];
    let target = 2;
    assert!(!Solution::search_matrix(matrix, target));

    let matrix = vec![vec![1], vec![3]];
    let target = 3;
    assert!(Solution::search_matrix(matrix, target));

    let matrix = vec![vec![1, 3]];
    let target = 3;
    assert!(Solution::search_matrix(matrix, target));
}
