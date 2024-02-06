/*
* @lc app=leetcode id=155 lang=rust
*
* [155] Min Stack
*
* https://leetcode.com/problems/min-stack/description/
*
* algorithms
* Medium (53.43%)
* Likes:    13617
* Dislikes: 831
* Total Accepted:    1.5M
* Total Submissions: 2.9M
* Testcase Example:  '["MinStack","push","push","push","getMin","pop","top","getMin"]\n' +
 '[[],[-2],[0],[-3],[],[],[],[]]'
*
* Design a stack that supports push, pop, top, and retrieving the minimum
* element in constant time.
*
* Implement the MinStack class:
*
*
* MinStack() initializes the stack object.
* void push(int val) pushes the element val onto the stack.
* void pop() removes the element on the top of the stack.
* int top() gets the top element of the stack.
* int getMin() retrieves the minimum element in the stack.
*
*
* You must implement a solution with O(1) time complexity for each
* function.
*
*
* Example 1:
*
*
* Input
* ["MinStack","push","push","push","getMin","pop","top","getMin"]
* [[],[-2],[0],[-3],[],[],[],[]]
*
* Output
* [null,null,null,null,-3,null,0,-2]
*
* Explanation
* MinStack minStack = new MinStack();
* minStack.push(-2);
* minStack.push(0);
* minStack.push(-3);
* minStack.getMin(); // return -3
* minStack.pop();
* minStack.top();    // return 0
* minStack.getMin(); // return -2
*
*
*
* Constraints:
*
*
* -2^31 <= val <= 2^31 - 1
* Methods pop, top and getMin operations will always be called on non-empty
* stacks.
* At most 3 * 10^4 calls will be made to push, pop, top, and getMin.
*
*
*/

// @lc code=start

use std::cmp;

pub struct MinStack {
    elements: Vec<i32>,
    minstack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            minstack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        let min = cmp::min(*self.minstack.last().unwrap_or(&i32::MAX), val);
        self.minstack.push(min);
        self.elements.push(val);
    }

    pub fn pop(&mut self) {
        self.minstack.pop();
        self.elements.pop();
    }

    pub fn top(&self) -> i32 {
        *self.elements.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.minstack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @lc code=end

#[test]
fn test_min_stack() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    assert_eq!(stack.get_min(), -3);
    stack.pop();
    assert_eq!(stack.top(), 0);
    assert_eq!(stack.get_min(), -2);
}
