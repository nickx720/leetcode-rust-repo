impl Solution {
    fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut stack :VecDeque<i32> = VecDeque::new();
        let k = k as usize;
        let mut moves = nums.len() - k;
        for x in nums {
            while !stack.is_empty() && moves != 0 && *stack.front().unwrap() > x {
                stack.pop_front();
                moves -=1;
            }
            stack.push_front(x);
        }
        stack.into_iter().rev().take(k).collect()
    }

}
