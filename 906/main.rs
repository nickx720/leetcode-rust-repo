/// https://dev.to/seanpgallivan/solution-super-palindromes-30d
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left = left.parse::<u128>().unwrap();
        let right = right.parse::<u128>().unwrap();
        let mut ans = if 9 >= left && 9 <= right { 1 } else { 0 };

        fn is_pal(value: &str) -> bool {
            let value = value.chars().collect::<Vec<char>>();
            let (mut i, mut j) = (0, value.len() - 1);
            while i < j {
                if value[i] != value[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        fn format_radix(mut x: u32) -> String {
            let mut result = vec![];

            loop {
                let m = x % 3;
                x = x / 3;

                result.push(std::char::from_digit(m, 3).unwrap());
                if x == 0 {
                    break;
                }
            }
            result.into_iter().rev().collect()
        }

        for i in 1..19684 {
            let num = format_radix(i);
            if is_pal(&num[..]) {
                let square = num.parse::<u128>().unwrap() * num.parse::<u128>().unwrap();
                if square > right {
                    return ans;
                }
                if square >= left && is_pal(&square.to_string()) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
