impl Solution{
    fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for (index,item) in nums.iter().enumerate(){
            for next_item in nums.iter().skip(index){
                if (item - next_item).abs() == k {
                    count+=1;
                }
            }
        }
        count
    }
}
