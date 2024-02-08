/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */
pub struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut table: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for str in strs {
            table.entry(Self::freqs(&str)).or_default().push(str);
        }
        let mut res: Vec<Vec<String>> = Vec::new();
        for mut v in table.into_values() {
            v.sort();
            res.push(v);
        }

        res.sort_by_key(|a| a.len());
        res
    }

    fn freqs(s: &str) -> [i32; 26] {
        let mut freqs = [0; 26];
        for b in s.bytes() {
            freqs[(b - b'a') as usize] += 1;
        }
        freqs
    }
}
// @lc code=end

#[test]
fn test_group_anagrams() {
    use crate::tests::StringVec;

    let case1 = StringVec::from(["eat", "tea", "tan", "ate", "nat", "bat"]);
    let ans1: Vec<Vec<String>> = vec![
        StringVec::from(["bat"]),
        StringVec::from(["nat", "tan"]),
        StringVec::from(["ate", "eat", "tea"]),
    ];
    assert_eq!(Solution::group_anagrams(case1), ans1);
}
