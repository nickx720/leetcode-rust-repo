impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let arabics: Vec<i32> = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let roman: Vec<&str> = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut output = String::new();
        for i in 0..arabics.len() {
            while num - arabics[i] >= 0 {
                output.push_str(roman[i]);
                num -= arabics[i];
            }
        }
        output
    }
}
