impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map:HashMap<i32,i32> = HashMap::new();
        for item in arr {
            *map.entry(item).or_insert(0)+=1;
        }
        let output = map.into_iter().filter(|(x,y)| x == y ).collect::<Vec<(i32,i32)>>().into_iter().max();
        if let Some((value,_)) = output {
            value
        } else {
            -1
        }

    }
}
