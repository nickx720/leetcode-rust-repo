impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let repeated_times = a.len() / 2;
        let mut count_of_items: HashMap<i32, i32> = HashMap::new();
        for item in &a {
            let count = count_of_items.entry(*item).or_insert(0);
            *count += 1;
        }
        let mut output = 0;
        for (&key, &value) in &count_of_items {
            if value == repeated_times as i32 {
                output = key;
            }
        }
        output
    }
}
