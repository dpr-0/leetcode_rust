/*
 * @lc app=leetcode id=1897 lang=rust
 *
 * [1897] Redistribute Characters to Make All Strings Equal
 */
pub struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        if words.len() == 1 {
            return true;
        }
        let mut counter: HashMap<char, u32> = HashMap::new();
        for word in words.iter() {
            for char in word.chars() {
                match counter.get_mut(&char) {
                    Some(c) => *c += 1,
                    None => {
                        counter.insert(char, 1);
                    }
                }
            }
        }
        for (_, v) in counter.into_iter() {
            let num_word = words.len() as u32;
            if v < num_word || v % num_word != 0 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[test]
fn test_make_equal() {
    use crate::tests::StringVec;

    let case1 = StringVec::from(["abc", "aabc", "bc"]);
    assert!(Solution::make_equal(case1));

    let case2 = StringVec::from(["ab", "a"]);
    assert!(!Solution::make_equal(case2));

    let case3 = StringVec::from([
        "caaaaa",
        "aaaaaaaaa",
        "a",
        "bbb",
        "bbbbbbbbb",
        "bbb",
        "cc",
        "cccccccccccc",
        "ccccccc",
        "ccccccc",
        "cc",
        "cccc",
        "c",
        "cccccccc",
        "c",
    ]);
    assert!(Solution::make_equal(case3));
}
