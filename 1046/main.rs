impl Solution
{
    fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.len() == 1 {
            return stones[0];
        }
        let mut stones = stones;
        stones.sort();
        while stones.len() > 1 {
            let (last_val,prev_last_val) = (stones.pop().unwrap(),stones.pop().unwrap());
            if last_val != prev_last_val {
                let value = (last_val - prev_last_val).abs();
                stones.push(value);
                stones.sort();
            }else {
                stones.push(0);
            }
        }
        stones[0]
    }

}
