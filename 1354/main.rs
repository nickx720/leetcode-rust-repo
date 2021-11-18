// https://dev.to/seanpgallivan/solution-construct-target-array-with-multiple-sums-24d4
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;
        let mut queue: BinaryHeap<i32> = BinaryHeap::new();
        let mut sum = target.iter().sum::<i32>();
        target.iter().for_each(|&element| queue.push(element));

        while *queue.peek().unwrap() != 1 {
            let mut num = queue.pop().unwrap();
            sum -= num;
            if num <= sum || sum < 1 {
                return false;
            }
            num %= sum;
            sum += num;
            if num > 0 {
                queue.push(num);
            } else {
                queue.push(sum);
            }
        }
        true
    }
}
