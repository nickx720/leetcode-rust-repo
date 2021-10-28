/// https://dev.to/seanpgallivan/solution-triangle-1lg0
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;
        if triangle.len() == 1 {
            return triangle[0][0]
        }
        for i in (0..=triangle.len()-2).rev(){
            for j in (0..=triangle[i].len()-1).rev(){
                triangle[i][j] += std::cmp::min(triangle[i+1][j],triangle[i+1][j+1]);
            }
        }
        triangle[0][0]

    }
}
