impl Solution {
fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans:Vec<i32> = Vec::new();
    for &item in nums.iter(){
        let item = item as usize;
        ans.push(nums[item]);
    }
    ans
}
}
