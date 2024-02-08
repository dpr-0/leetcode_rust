/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */
pub struct Solution;

// @lc code=start

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = Vec::new();
        for w in s.split_whitespace().rev() {
            res.push(w);
        }
        res.join(" ")
    }
}
// @lc code=end

#[test]
fn test_reverse_words() {
    let case1 = "the sky is blue";
    assert_eq!(
        Solution::reverse_words(case1.to_string()),
        "blue is sky the"
    );
}
