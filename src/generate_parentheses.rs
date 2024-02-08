/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 *
 * https://leetcode.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (73.78%)
 * Likes:    20426
 * Dislikes: 870
 * Total Accepted:    1.7M
 * Total Submissions: 2.3M
 * Testcase Example:  '3'
 *
 * Given n pairs of parentheses, write a function to generate all combinations
 * of well-formed parentheses.
 *
 *
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 8
 *
 *
 */

pub struct Solution;
// @lc code=start

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        let mut res: Vec<String> = Vec::new();

        fn backtrack(
            stack: &mut Vec<String>,
            res: &mut Vec<String>,
            n: i32,
            num_open: i32,
            num_closed: i32,
        ) {
            if num_open == n && num_closed == n {
                res.push(stack.join(""));
                return;
            }
            if num_open < n {
                stack.push("(".to_owned());
                backtrack(stack, res, n, num_open + 1, num_closed);
                stack.pop();
            }
            if num_closed < num_open {
                stack.push(")".to_owned());
                backtrack(stack, res, n, num_open, num_closed + 1);
                stack.pop();
            }
        }
        backtrack(&mut stack, &mut res, n, 0, 0);

        res
    }
}
// @lc code=end

#[test]
fn test_generate_parenthesis() {
    let case1 = 3;
    assert_eq!(
        Solution::generate_parenthesis(case1),
        ["((()))", "(()())", "(())()", "()(())", "()()()"]
    );

    let case2 = 1;
    assert_eq!(Solution::generate_parenthesis(case2), ["()"]);
}
