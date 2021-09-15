impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut ans = 0;
        for item in arr1 {
            if !arr2.iter().any(|curr| (curr -item).abs() <= d) {
                ans+=1;
            }
        }
        ans

    }
}
