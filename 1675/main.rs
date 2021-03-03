impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut priority_queue = BinaryHeap::new();
        for &item in nums.iter() {
            let mut n = item;
            if n % 2 != 0 {
                n *= 2;
            }
            priority_queue.push(n);
        }
        // Get minimum from max priority queue
        let mut minimum = *priority_queue.iter().min().unwrap();
        let mut output = std::i32::MAX;
        while let Some(max_val) = priority_queue.pop() {
            output = std::cmp::min(output, max_val - minimum);
            if max_val % 2 == 0 {
                minimum = std::cmp::min(minimum, max_val / 2);
                priority_queue.push(max_val / 2);
            } else {
                break;
            }
        }
        output
    }
}
