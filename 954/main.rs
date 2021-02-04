impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        for &item in &arr {
            let count = frequency.entry(item).or_insert(0);
            *count += 1;
        }
        let mut arr = arr;
        arr.sort_by(|a, b| a.abs().cmp(&b.abs()));
        for item in &arr {
            if let Some(&count) = frequency.get(item) {
                if count == 0 {
                    continue;
                }
            }
            if let Some(&count) = frequency.get(&(*item * 2)) {
                if count == 0 {
                    return false;
                }
            } else {
                // element is not in the list
                return false;
            }
            if let Some(count) = frequency.get_mut(item) {
                *count -= 1;
            }
            if let Some(count) = frequency.get_mut(&(*item * 2)) {
                *count -= 1;
            }
        }
        true
    }
}
