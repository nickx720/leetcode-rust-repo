/// https://dev.to/seanpgallivan/solution-brick-wall-4jld
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut freq:HashMap<i32,i32> = HashMap::new();
        for i in 0..wall.len(){
            let row = &wall[i];
            let mut row_sum = row[0];
            for j in 1..row.len(){
                freq.insert(row_sum, freq.get(&row_sum).unwrap_or(&0) + 1);
                row_sum+=row[j];
            }
        }
        let mut best = 0;
        for (_,value) in freq {
            if value > best {
                best = value;
            }
        }
        let output = wall.len() as i32 - best;
        output

    }
}
