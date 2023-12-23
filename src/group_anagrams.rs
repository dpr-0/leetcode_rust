/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */
use crate::Solution;

// @lc code=start
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut table: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, str) in strs.iter().enumerate() {
            let mut str_vec: Vec<char> = str.chars().collect();
            str_vec.sort();
            let sorted_str: String = str_vec.into_iter().collect();
            if table.contains_key(&sorted_str) {
                if let Some(indexes) = table.get_mut(&sorted_str) {
                    indexes.push(i);
                }
            } else {
                table.insert(sorted_str, vec![i]);
            }
        }
        let mut res: Vec<Vec<String>> = Vec::new();
        for (_, i_group) in table.into_iter() {
            let mut g: Vec<String> = Vec::new();
            for i in i_group {
                g.push(strs[i].clone());
            }
            g.sort();
            res.push(g);
        }
        res.sort_by(|a, b| a.len().cmp(&b.len()));
        res
    }
}
// @lc code=end

#[test]
fn test_group_anagrams() {
    let case1: Vec<String> = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .map(String::from)
        .to_vec();
    let ans1: Vec<Vec<String>> = vec![
        ["bat"].map(String::from).to_vec(),
        ["nat", "tan"].map(String::from).to_vec(),
        ["ate", "eat", "tea"].map(String::from).to_vec(),
    ];
    assert_eq!(Solution::group_anagrams(case1), ans1);
}
