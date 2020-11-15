impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut dictionary: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for val in 0..nums.len() {
            if dictionary.contains_key(&nums[val]) {
                dictionary.insert(nums[val], dictionary[&nums[val]] + 1);
            } else {
                dictionary.insert(nums[val], 1);
            }
        }
        let mut count_vec: Vec<i32> = Vec::new();
        // creating vector from hashmap
        let mut v: Vec<_> = std::iter::FromIterator::from_iter(dictionary);
        // Sorting the vector based on number of values https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
        v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        for (key, _) in v.iter() {
            count_vec.push(*key);
        }
        (&count_vec[0..k as usize]).to_vec()
    }
}
