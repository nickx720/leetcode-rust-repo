impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut char_map: Vec<i32> = vec![0; 26];
        for &c in tasks.iter() {
            char_map[c as usize - 'A' as usize] += 1;
        }
        char_map.sort();
        let max_val = char_map[25] - 1;
        let mut idle_slot = max_val * n;
        for i in (0..=24 as usize).rev() {
            idle_slot -= std::cmp::min(char_map[i], max_val);
        }
        let output = if idle_slot > 0 {
            idle_slot + tasks.len() as i32
        } else {
            tasks.len() as i32
        };
        output
    }
}
