impl Solution {
    fn calculate(s: String) -> i32 {
        let mut queue: std::collections::VecDeque<char> = std::collections::VecDeque::new();
        for c in s.chars() {
            if c != ' ' {
                queue.push_back(c);
            }
        }
        queue.push_back(' ');
        let output = recursivecalculate(&mut queue);
        output
    }

    fn recursivecalculate(queue: &mut std::collections::VecDeque<char>) -> i32 {
        let mut nums: i32 = 0;
        let mut sum: i32 = 0;
        let mut prev: i32 = 0;
        let mut prev_op: char = '+';
        while !queue.is_empty() {
            let current_char: char = queue.pop_front().unwrap();
            if current_char >= '0' && current_char <= '9' {
                nums = nums * 10 + (current_char.to_digit(10).unwrap_or(0) as i32) - 0;
            } else if current_char == '(' {
                nums = recursivecalculate(queue)
            } else {
                match prev_op {
                    '+' => {
                        sum += prev;
                        prev = nums
                    }
                    '-' => {
                        sum += prev;
                        prev = -nums
                    }
                    '*' => prev *= nums,
                    '/' => prev /= nums,
                    _ => println!("cannot compute"),
                }
                if current_char == ')' {
                    break;
                }
                nums = 0;
                prev_op = current_char;
            }
        }
        sum + prev
    }
}
