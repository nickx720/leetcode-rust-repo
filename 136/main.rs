impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        let mut output: i32 = 0;
        let mut singe_number : std::collections::HashMap<i32,i32> = std::collections::HashMap::new();
        for val in nums {        
            if singe_number.contains_key(&val) {
                singe_number.insert(val, 1);
            } else {
                singe_number.insert(val, 0);
            }
        }
        for (k,v) in singe_number.drain(){
            if v ==0 {
                output = k;
                break;
            }
        }    
        output
        /* 
        Uses XOR property that a ^ a = 0 and a ^ 0 = a
        pub fn single_number(nums: Vec<i32>) -> i32 {        
            nums.iter().skip(1).fold(nums[0], |acc, x| acc ^ x)
        } */
    }
}