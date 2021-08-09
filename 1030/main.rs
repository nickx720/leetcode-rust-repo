impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut return_value : Vec<Vec<i32>> = Vec::new();
        for item in 0..r {
            for item_second in 0..c{
                let dist = (r0-item).abs() + (c0-item_second).abs();
                return_value.push(vec![item,item_second,dist]);
            }
        }
        return_value.sort_by(|a,b| a[2].cmp(&b[2]));
        return_value.into_iter().map(|mut x| {
            x.pop();
            x
        }).collect()

    }
}
