/*
 * @lc app=leetcode id=875 lang=rust
 *
 * [875] Koko Eating Bananas
 *
 * https://leetcode.com/problems/koko-eating-bananas/description/
 *
 * algorithms
 * Medium (49.41%)
 * Likes:    9962
 * Dislikes: 551
 * Total Accepted:    569.3K
 * Total Submissions: 1.2M
 * Testcase Example:  '[3,6,7,11]\n8'
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has
 * piles[i] bananas. The guards have gone and will come back in h hours.
 *
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she
 * chooses some pile of bananas and eats k bananas from that pile. If the pile
 * has less than k bananas, she eats all of them instead and will not eat any
 * more bananas during this hour.
 *
 * Koko likes to eat slowly but still wants to finish eating all the bananas
 * before the guards return.
 *
 * Return the minimum integer k such that she can eat all the bananas within h
 * hours.
 *
 *
 * Example 1:
 *
 *
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 *
 *
 * Example 3:
 *
 *
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= piles.length <= 10^4
 * piles.length <= h <= 10^9
 * 1 <= piles[i] <= 10^9
 *
 *
 */
pub struct Solution;
// @lc code=start
use std::cmp;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut left, mut right) = (1, *piles.iter().max().unwrap());
        let mut min_k = right;
        while left <= right {
            let k = left + (right - left) / 2;
            let hours: i64 = piles
                .iter()
                .map(|p| (*p as u32).div_ceil(k as u32) as i64)
                .sum();

            if hours <= h as i64 {
                min_k = cmp::min(min_k, k);
                right = k - 1;
            } else {
                left = k + 1;
            }
        }
        min_k
    }
}
// @lc code=end

#[test]
fn test_min_eating_speed() {
    let (piles, h) = (vec![3, 6, 7, 11], 8);
    assert_eq!(Solution::min_eating_speed(piles, h), 4);

    let (piles, h) = (vec![30, 11, 23, 4, 20], 5);
    assert_eq!(Solution::min_eating_speed(piles, h), 30);

    let (piles, h) = (vec![30, 11, 23, 4, 20], 6);
    assert_eq!(Solution::min_eating_speed(piles, h), 23);

    let (piles, h) = (vec![805306368, 805306368, 805306368], 1000000000);
    assert_eq!(Solution::min_eating_speed(piles, h), 3);
}
