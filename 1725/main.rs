impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut count: HashMap<i32, i32> = HashMap::new();
        for item in rectangles {
            let min: i32 = *item.iter().min().unwrap();
            let counter = count.entry(min).or_insert(0);
            *counter += 1;
        }
        let (_, max) = count.into_iter().max().unwrap();
        max
    }
}
