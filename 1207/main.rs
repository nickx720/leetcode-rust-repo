impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut count_store: HashMap<i32, i32> = HashMap::new();
        for &item in arr.iter() {
            let count = count_store.entry(item).or_insert(0);
            *count += 1;
        }
        let mut count_set: HashSet<i32> = HashSet::new();
        for (_, &value) in &count_store {
            count_set.insert(value);
        }
        count_store.len() == count_set.len()
    }
}
