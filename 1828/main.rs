impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut output : Vec<i32> = Vec::new();
        for query in queries{
            let mut count = 0;
            for point in points.iter(){
                let (x_1,y_1,radius) = (query[0] as f64,query[1] as f64,query[2] as f64);
                let (x_2,y_2) = (point[0] as f64,point[1] as f64);
                let diff = ((x_1 - x_2).powi(2) + (y_1 - y_2).powi(2));
                if diff <= radius.powi(2){
                    count+=1;
                }

            }
            output.push(count);
        }
        output

    }
}
