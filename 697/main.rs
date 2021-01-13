impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut min_length = 0;
        let mut degree = 0;
        let mut first_occurence: std::collections::HashMap<i32, usize> =
            std::collections::HashMap::new();
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            if !first_occurence.contains_key(&num) {
                first_occurence.insert(num, idx);
            }
            let count = map.entry(num).or_insert(0);
            *count += 1;
            if let Some(&count_curr) = map.get(&num) {
                if degree < count_curr {
                    degree = count_curr;
                    min_length = (idx - *first_occurence.get(&num).unwrap()) as i32 + 1;
                } else if count_curr == degree {
                    min_length = std::cmp::min(
                        min_length,
                        (idx - *first_occurence.get(&num).unwrap()) as i32 + 1,
                    )
                }
            }
        }
        min_length
    }
}
