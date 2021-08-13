impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let max_no_of_candies = candy_type.len() / 2;
        let unique_candies: HashSet<i32> = HashSet::from_iter(candy_type);
        if max_no_of_candies > unique_candies.len(){
            unique_candies.len() as i32
        } else {
            max_no_of_candies as i32
        }

    }
}
