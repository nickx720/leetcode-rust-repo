impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut inventory = inventory.iter().map(|&v| v as i64).collect::<Vec<i64>>();
        inventory.sort_by(|a, b| b.cmp(a));
        let mut orders = orders as i64;
        let mut highest = inventory[0];
        let mut index = 0;
        let mut answer = 0;
        for i in 1..inventory.len() {
            let batch_size = (i as i64) * (highest - inventory[i]);
            if batch_size < orders {
                orders -= batch_size;
                answer += (i as i64) * (highest + inventory[i] + 1) * (highest - inventory[i]) / 2;
                highest = inventory[i];
            } else {
                break;
            }
            index = i;
        }
        let i = (index + 1) as i64;
        let h = orders / i;
        answer += i * h * (highest + (highest - h) + 1) / 2;
        orders = orders % i;
        highest = highest - h;
        answer += orders * highest;
        orders = 0;
        (answer % (10u32.pow(9) as i64 + 7)) as i32
    }
}
