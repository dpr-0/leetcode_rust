/*
 * @lc app=leetcode id=2114 lang=rust
 *
 * [2114] Maximum Number of Words Found in Sentences
 *
 * https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/description/
 *
 * algorithms
 * Easy (86.36%)
 * Likes:    1683
 * Dislikes: 55
 * Total Accepted:    255.1K
 * Total Submissions: 295.3K
 * Testcase Example:  '["alice and bob love leetcode","i think so too","this is great thanks very much"]'
 *
 * A sentence is a list of words that are separated by a single space with no
 * leading or trailing spaces.
 *
 * You are given an array of strings sentences, where each sentences[i]
 * represents a single sentence.
 *
 * Return the maximum number of words that appear in a single sentence.
 *
 *
 * Example 1:
 *
 *
 * Input: sentences = ["alice and bob love leetcode", "i think so too", "this
 * is great thanks very much"]
 * Output: 6
 * Explanation:
 * - The first sentence, "alice and bob love leetcode", has 5 words in total.
 * - The second sentence, "i think so too", has 4 words in total.
 * - The third sentence, "this is great thanks very much", has 6 words in
 * total.
 * Thus, the maximum number of words in a single sentence comes from the third
 * sentence, which has 6 words.
 *
 *
 * Example 2:
 *
 *
 * Input: sentences = ["please wait", "continue to fight", "continue to win"]
 * Output: 3
 * Explanation: It is possible that multiple sentences contain the same number
 * of words.
 * In this example, the second and third sentences (underlined) have the same
 * number of words.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= sentences.length <= 100
 * 1 <= sentences[i].length <= 100
 * sentences[i] consists only of lowercase English letters and ' ' only.
 * sentences[i] does not have leading or trailing spaces.
 * All the words in sentences[i] are separated by a single space.
 *
 *
 */
pub struct Solution {}
// @lc code=start
use std::cmp;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .into_iter()
            .map(|s| s.split(' ').count() as i32)
            .reduce(cmp::max)
            .unwrap()
    }
}
// @lc code=end

#[test]
fn test_most_words_found() {
    let case1 = vec![
        String::from("alice and bob love leetcode"),
        String::from("i think so too"),
        String::from("this is great thanks very much"),
    ];
    assert_eq!(Solution::most_words_found(case1), 6);

    let case2 = vec![
        String::from("please wait"),
        String::from("continue to fight"),
        String::from("continue to win"),
    ];
    assert_eq!(Solution::most_words_found(case2), 3);
}
