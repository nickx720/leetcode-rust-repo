use std::collections::HashMap;
impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counter:HashMap<i32,i32> = HashMap::new();
        Self::add_to_collection(&mut counter, nums1);
        Self::add_to_collection(&mut counter, nums2);
        Self::add_to_collection(&mut counter, nums3);
        counter.into_iter().filter(|&(_,value)| value >= 2).map(|(value,_)| value).collect()

    }
    fn add_to_collection(counter: &mut HashMap<i32,i32>, nums: Vec<i32>) {
        let mut nums = nums;
        nums.sort();
        nums.dedup();
        nums.iter().for_each(|&item| {
            *counter.entry(item).or_insert(0)+=1;
        })
    }

}
