impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        fn get_board_value(board: &Vec<Vec<i32>>, num: i32) -> i32 {
            let n = board.len() as i32;
            let r = (num - 1) / n;
            let x = n - 1 - r;
            let y = if r % 2 == 0 {
                num - 1 - r * n
            } else {
                n + r * n - num
            };
            board[x as usize][y as usize]
        }
        let mut queue: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
        queue.push_back(1);
        let n = board.len();
        let mut visited: Vec<bool> = vec![false; n * n + 1];
        let mut ans = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some(num) = queue.pop_front() {
                    if visited[num as usize] {
                        continue;
                    }
                    visited[num as usize] = true;
                    if num == (n * n) as i32 {
                        return ans;
                    }
                    (1..=6).for_each(|i| {
                        let mut next = num + i;
                        if next <= (n * n) as i32 {
                            let value = get_board_value(&board, next);
                            if value > 0 {
                                next = value;
                            }
                            if !visited[next as usize] {
                                queue.push_back(next);
                            }
                        }
                    })
                }
            }
            ans += 1;
        }
        -1
    }
}
