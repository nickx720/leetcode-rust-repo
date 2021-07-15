impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let (mut y, mut ans) = (y,0);
        while x < y{
            ans+=1;
            if y %2 != 0 {
                y+=1;
            } else {
                y/=2;
            }
        }
        x - y + ans

    }
}
