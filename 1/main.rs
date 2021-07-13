impl Solution {
    // leetcode 1
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; 2];
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i in 0..nums.len() {
        let difference = target - nums[i];
        if map.contains_key(&difference) {
            result[0] = i as i32;
            result[1] = *map.get_key_value(&difference).unwrap().1;
        }
        map.insert(nums[i], i as i32);
    }
    result
}

}
