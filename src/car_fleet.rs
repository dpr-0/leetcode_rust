/*
 * @lc app=leetcode id=853 lang=rust
 *
 * [853] Car Fleet
 *
 * https://leetcode.com/problems/car-fleet/description/
 *
 * algorithms
 * Medium (50.48%)
 * Likes:    3306
 * Dislikes: 861
 * Total Accepted:    222.4K
 * Total Submissions: 439.4K
 * Testcase Example:  '12\n[10,8,0,5,3]\n[2,4,1,1,3]'
 *
 * There are n cars going to the same destination along a one-lane road. The
 * destination is target miles away.
 *
 * You are given two integer array position and speed, both of length n, where
 * position[i] is the position of the i^th car and speed[i] is the speed of the
 * i^th car (in miles per hour).
 *
 * A car can never pass another car ahead of it, but it can catch up to it and
 * drive bumper to bumper at the same speed. The faster car will slow down to
 * match the slower car's speed. The distance between these two cars is ignored
 * (i.e., they are assumed to have the same position).
 *
 * A car fleet is some non-empty set of cars driving at the same position and
 * same speed. Note that a single car is also a car fleet.
 *
 * If a car catches up to a car fleet right at the destination point, it will
 * still be considered as one car fleet.
 *
 * Return the number of car fleets that will arrive at the destination.
 *
 *
 * Example 1:
 *
 *
 * Input: target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
 * Output: 3
 * Explanation:
 * The cars starting at 10 (speed 2) and 8 (speed 4) become a fleet, meeting
 * each other at 12.
 * The car starting at 0 does not catch up to any other car, so it is a fleet
 * by itself.
 * The cars starting at 5 (speed 1) and 3 (speed 3) become a fleet, meeting
 * each other at 6. The fleet moves at speed 1 until it reaches target.
 * Note that no other cars meet these fleets before the destination, so the
 * answer is 3.
 *
 *
 * Example 2:
 *
 *
 * Input: target = 10, position = [3], speed = [3]
 * Output: 1
 * Explanation: There is only one car, hence there is only one fleet.
 *
 *
 * Example 3:
 *
 *
 * Input: target = 100, position = [0,2,4], speed = [4,2,1]
 * Output: 1
 * Explanation:
 * The cars starting at 0 (speed 4) and 2 (speed 2) become a fleet, meeting
 * each other at 4. The fleet moves at speed 2.
 * Then, the fleet (speed 2) and the car starting at 4 (speed 1) become one
 * fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches
 * target.
 *
 *
 *
 * Constraints:
 *
 *
 * n == position.length == speed.length
 * 1 <= n <= 10^5
 * 0 < target <= 10^6
 * 0 <= position[i] < target
 * All the values of position are unique.
 * 0 < speed[i] <= 10^6
 *
 *
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        cars.sort_by_key(|i| i.0);
        let mut stack: Vec<f32> = Vec::new();
        while let Some((p1, s1)) = cars.pop() {
            let t1 = (target - p1) as f32 / s1 as f32;
            if let Some(&t2) = stack.last() {
                if t1 > t2 {
                    // t1 won't meet t2
                    stack.push(t1);
                }
            } else {
                stack.push(t1);
            }
        }
        stack.len() as i32
    }
}
// @lc code=end

#[test]
fn test_car_fleet() {
    let (target, position, speed) = (12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
    assert_eq!(Solution::car_fleet(target, position, speed), 3);

    let (target, position, speed) = (10, vec![3], vec![3]);
    assert_eq!(Solution::car_fleet(target, position, speed), 1);

    let (target, position, speed) = (100, vec![0, 2, 4], vec![4, 2, 1]);
    assert_eq!(Solution::car_fleet(target, position, speed), 1);

    let (target, position, speed) = (10, vec![0, 2], vec![1, 1]);
    assert_eq!(Solution::car_fleet(target, position, speed), 2);

    let (target, position, speed) = (10, vec![0, 4, 2], vec![2, 1, 3]);
    assert_eq!(Solution::car_fleet(target, position, speed), 1);

    let (target, position, speed) = (13, vec![10, 2, 5, 7, 4, 6, 11], vec![7, 5, 10, 5, 9, 4, 1]);
    assert_eq!(Solution::car_fleet(target, position, speed), 2);
}
