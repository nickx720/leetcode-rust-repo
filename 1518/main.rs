impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let (mut total, mut num_bottles) = (0, num_bottles);
        while num_bottles >= num_exchange {
            num_bottles -= num_exchange;
            num_bottles += 1;
            total += num_exchange;
        }
        total + num_bottles
    }
}
