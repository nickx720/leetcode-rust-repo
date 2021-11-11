/// https://dev.to/seanpgallivan/solution-powerful-integers-1o2p
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut output:HashSet<i32> = HashSet::new();
        let mut xi = 1;
        while xi < bound{
            let mut yj =1;
            while xi+ yj <= bound{
                output.insert(xi+yj);
                if y == 1 {
                    break;
                }
                yj *=y;
            }
            if x == 1 {
                break;
            }
            xi *= x;
        }
        output.into_iter().collect()

    }
}
