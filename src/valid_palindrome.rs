/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (46.29%)
 * Likes:    8768
 * Dislikes: 8199
 * Total Accepted:    2.6M
 * Total Submissions: 5.6M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * A phrase is a palindrome if, after converting all uppercase letters into
 * lowercase letters and removing all non-alphanumeric characters, it reads the
 * same forward and backward. Alphanumeric characters include letters and
 * numbers.
 *
 * Given a string s, return true if it is a palindrome, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: s = " "
 * Output: true
 * Explanation: s is an empty string "" after removing non-alphanumeric
 * characters.
 * Since an empty string reads the same forward and backward, it is a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 2 * 10^5
 * s consists only of printable ASCII characters.
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let (mut left, mut right) = (0_usize, s.len() - 1);
        while left < right {
            if !s.as_bytes()[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            if !s.as_bytes()[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }
            if s.as_bytes()[left].to_ascii_lowercase() == s.as_bytes()[right].to_ascii_lowercase() {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[test]
fn test_is_palindrome() {
    let case1 = String::from("A man, a plan, a canal: Panama");
    assert!(Solution::is_palindrome(case1));

    let case2 = String::from("race a car");
    assert!(!Solution::is_palindrome(case2));

    let case3 = String::from(" ");
    assert!(Solution::is_palindrome(case3));

    let case4 = String::from("0P");
    assert!(!Solution::is_palindrome(case4));
}
