impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut unique: HashMap<i32, i32> = HashMap::new();
        for item in nums {
            let count = unique.entry(item).or_insert(0);
            *count += 1;
        }
        unique.iter().fold(
            0,
            |acc, (&key, &value)| if value == 1 { acc + key } else { acc + 0 },
        )
    }
}
