impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut count = 0;
        for item in operations{
            if item.contains("+") {
                count+=1;
            } else {
                count-=1;
            }
        }
        count

    }
}
