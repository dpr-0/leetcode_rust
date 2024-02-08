/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (40.28%)
 * Likes:    23172
 * Dislikes: 1629
 * Total Accepted:    4.2M
 * Total Submissions: 10.4M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * Every close bracket has a corresponding open bracket of the same type.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */
pub struct Solution;
// @lc code=start

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() < 2 {
            return false;
        }

        let mut stack: Vec<char> = Vec::new();
        for bracket in s.chars() {
            if let ')' | ']' | '}' = bracket {
                if let Some(left_bracket) = stack.pop() {
                    let pair = (left_bracket, bracket);
                    if !(pair == ('(', ')') || pair == ('[', ']') || pair == ('{', '}')) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                stack.push(bracket);
            }
        }

        stack.is_empty()
    }
}
// @lc code=end

#[test]
fn test_is_valid() {
    let case1 = String::from("()");
    assert_eq!(Solution::is_valid(case1), true);

    let case2 = String::from("()[]{}");
    assert_eq!(Solution::is_valid(case2), true);

    let case3 = String::from("(]");
    assert_eq!(Solution::is_valid(case3), false);

    let case4 = String::from("(");
    assert_eq!(Solution::is_valid(case4), false);

    let case5 = String::from(")(){}");
    assert_eq!(Solution::is_valid(case5), false);
}
