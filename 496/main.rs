impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
            use std::collections::{HashMap,VecDeque};
    let mut output:Vec<i32> = Vec::new();
    let mut next_greatest:HashMap<i32,i32> = HashMap::new();
    let mut stack:VecDeque<i32> = VecDeque::new();
    for item in nums2{
        while !stack.is_empty() && *stack.back().unwrap() < item {
            next_greatest.insert(stack.pop_back().unwrap(), item);
        }
        stack.push_back(item);
    }
    for item in nums1.iter() {
        let item = next_greatest.get(item);
        if item.is_some(){
            output.push(*item.unwrap());
        } else {
            output.push(-1);
        }
    }
    output

    }
}
