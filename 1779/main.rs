impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let points_filtered = points.iter().filter(|&item| {
            if *item.first().unwrap() == x || *item.last().unwrap() == y {
                true
            } else {
                false
            }
        }).collect::<Vec<_>>();
        if points_filtered.len() == 0 {
            return -1;
        };

        let mut min = (i32::MAX,0);
        points.iter().enumerate().for_each(|(index,item)|{
            let (first,second) = (*item.first().unwrap(),*item.last().unwrap());
            if x == first || y == second {
                let distance = (x - first).abs() + (y - second).abs();
                if distance < min.0 {
                    min = (distance,index)
                }
            }

        });
        let (_,output) = min;
        output as i32

    }
}
