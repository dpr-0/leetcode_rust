/*
 * @lc app=leetcode id=150 lang=rust
 *
 * [150] Evaluate Reverse Polish Notation
 *
 * https://leetcode.com/problems/evaluate-reverse-polish-notation/description/
 *
 * algorithms
 * Medium (48.35%)
 * Likes:    7380
 * Dislikes: 1047
 * Total Accepted:    958.2K
 * Total Submissions: 1.9M
 * Testcase Example:  '["2","1","+","3","*"]'
 *
 * You are given an array of strings tokens that represents an arithmetic
 * expression in a Reverse Polish Notation.
 *
 * Evaluate the expression. Return an integer that represents the value of the
 * expression.
 *
 * Note that:
 *
 *
 * The valid operators are '+', '-', '*', and '/'.
 * Each operand may be an integer or another expression.
 * The division between two integers always truncates toward zero.
 * There will not be any division by zero.
 * The input represents a valid arithmetic expression in a reverse polish
 * notation.
 * The answer and all the intermediate calculations can be represented in a
 * 32-bit integer.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: tokens = ["2","1","+","3","*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 *
 *
 * Example 2:
 *
 *
 * Input: tokens = ["4","13","5","/","+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 *
 *
 * Example 3:
 *
 *
 * Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
 * Output: 22
 * Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= tokens.length <= 10^4
 * tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the
 * range [-200, 200].
 *
 *
 */
pub struct Solution {}
// @lc code=start
impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        tokens.reverse();
        let mut operands: Vec<i32> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token.as_str() {
                "+" => {
                    let (right_operand, left_operand) =
                        (operands.pop().unwrap(), operands.pop().unwrap());
                    operands.push(left_operand + right_operand);
                }
                "-" => {
                    let (right_operand, left_operand) =
                        (operands.pop().unwrap(), operands.pop().unwrap());
                    operands.push(left_operand - right_operand);
                }
                "*" => {
                    let (right_operand, left_operand) =
                        (operands.pop().unwrap(), operands.pop().unwrap());
                    operands.push(left_operand * right_operand);
                }
                "/" => {
                    let (right_operand, left_operand) =
                        (operands.pop().unwrap(), operands.pop().unwrap());
                    operands.push(left_operand / right_operand);
                }
                operand => operands.push(operand.parse::<i32>().unwrap()),
            }
        }

        operands.pop().unwrap()
    }
}
// @lc code=end

#[test]
fn test_eval_rpn() {
    let case1: Vec<String> = ["2", "1", "+", "3", "*"]
        .into_iter()
        .map(String::from)
        .collect();
    assert_eq!(Solution::eval_rpn(case1), 9);

    let case2: Vec<String> = ["4", "13", "5", "/", "+"]
        .into_iter()
        .map(String::from)
        .collect();
    assert_eq!(Solution::eval_rpn(case2), 6);

    let case3: Vec<String> = [
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    assert_eq!(Solution::eval_rpn(case3), 22);
}
