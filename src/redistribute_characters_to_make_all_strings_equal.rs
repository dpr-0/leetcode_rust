/*
 * @lc app=leetcode id=1897 lang=rust
 *
 * [1897] Redistribute Characters to Make All Strings Equal
 */
pub struct Solution {}

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
    let case1 = vec!["abc", "aabc", "bc"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(Solution::make_equal(case1), true);

    let case2 = vec!["ab", "a"].iter().map(|x| x.to_string()).collect();
    assert_eq!(Solution::make_equal(case2), false);

    let case3 = vec![
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
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    assert_eq!(Solution::make_equal(case3), true);
}
