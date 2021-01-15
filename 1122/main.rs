impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        let mut count: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for &num in arr1.iter() {
            let counter = count.entry(num).or_insert(0);
            *counter += 1;
        }
        for &item in arr2.iter() {
            if count.contains_key(&item) {
                if let Some(&count_of_rep) = count.get(&item) {
                    for _ in 0..count_of_rep {
                        output.push(item);
                    }
                }
                count.remove(&item);
            }
        }
        let mut vel = Vec::new();
        for (&key, &val) in &count {
            for _ in 0..val {
                vel.push(key);
            }
        }
        vel.sort();
        output.append(&mut vel);
        output
    }
}
