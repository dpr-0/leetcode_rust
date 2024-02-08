/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 *
 * https://leetcode.com/problems/daily-temperatures/description/
 *
 * algorithms
 * Medium (65.91%)
 * Likes:    12781
 * Dislikes: 298
 * Total Accepted:    891.6K
 * Total Submissions: 1.4M
 * Testcase Example:  '[73,74,75,71,69,72,76,73]'
 *
 * Given an array of integers temperatures represents the daily temperatures,
 * return an array answer such that answer[i] is the number of days you have to
 * wait after the i^th day to get a warmer temperature. If there is no future
 * day for which this is possible, keep answer[i] == 0 instead.
 *
 *
 * Example 1:
 * Input: temperatures = [73,74,75,71,69,72,76,73]
 * Output: [1,1,4,2,1,1,0,0]
 * Example 2:
 * Input: temperatures = [30,40,50,60]
 * Output: [1,1,1,0]
 * Example 3:
 * Input: temperatures = [30,60,90]
 * Output: [1,1,0]
 *
 *
 * Constraints:
 *
 *
 * 1 <= temperatures.length <= 10^5
 * 30 <= temperatures[i] <= 100
 *
 *
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::with_capacity(temperatures.len());
        let mut res: Vec<i32> = vec![0; temperatures.len()];
        for (i2, &t) in temperatures.iter().enumerate() {
            while !stack.is_empty() && t > stack.last().unwrap().0 {
                let (_, i1) = stack.pop().unwrap();
                res[i1] = (i2 - i1) as i32;
            }
            stack.push((t, i2));
        }
        res
    }
}
// @lc code=end

#[test]
fn test_daily_temperatures() {
    let case1 = vec![73, 74, 75, 71, 69, 72, 76, 73];
    assert_eq!(
        Solution::daily_temperatures(case1),
        [1, 1, 4, 2, 1, 1, 0, 0]
    );

    let case2 = vec![30, 40, 50, 60];
    assert_eq!(Solution::daily_temperatures(case2), [1, 1, 1, 0]);

    let case3 = vec![30, 60, 90];
    assert_eq!(Solution::daily_temperatures(case3), [1, 1, 0]);
}
