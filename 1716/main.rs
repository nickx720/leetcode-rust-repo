impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let (mut week_value,mut day_value,mut total_value) = (0,0,0);
        for i in 1..=n{
            week_value = i / 7;
            day_value = i % 7;
            if day_value == 0 {
                day_value = 6;
            }
            total_value+=day_value+week_value;

        }
        total_value

    }
}
