impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut count_map: HashMap<i32,i32> = HashMap::new();
        let length = edges.len() as i32;

        for edge in edges{
            for item in edge {
                *count_map.entry(item).or_insert(0)+=1;
            }
        }

        for (key,value) in count_map {
            if value == length {
                return key;
            }
        }
        unreachable!()

    }
}
