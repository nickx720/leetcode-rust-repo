impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by(|a,b| a.get(0).cmp(&b.get(0)));
        let mut result = -1;
        for i in 1..points.len(){
            let diff = points[i][0] - points[i-1][0];
            result = std::cmp::max(diff,result);
        }
        result

    }
}
