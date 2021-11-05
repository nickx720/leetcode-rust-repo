impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        arr.iter().for_each(|s| {
            *map.entry(s).or_insert(0)+=1;
        });
        arr.iter()
            .filter(|s| map[s] == 1)
            .nth((k-1) as usize)
            .unwrap_or(&"".to_string())
            .to_string()

    }
}
