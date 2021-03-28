impl Solution {
    fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; t.len()];
        let mut stack: Vec<(i32, i32)> = Vec::new();
        for (index, &item) in t.iter().rev().enumerate() {
            while !stack.is_empty() && item >= stack[stack.len() - 1].0 {
                stack.pop();
            }
            if !stack.is_empty() && item < stack[stack.len() - 1].0 {
                result[index] = (stack[stack.len() - 1].1 - (index as i32)).abs();
            }
            stack.push((item, index as i32))
        }
        result.reverse();
        result
    }
}
