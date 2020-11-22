impl Solution {
    fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let output: Vec<i32> = nums
            .iter()
            .map(|x| {
                let mut standin = nums.clone();
                let index = standin.iter().position(|y| *y == *x).unwrap();
                standin.remove(index);
                let product: i32 = standin.iter().product();
                product
            })
            .collect();
        output
    }
}
