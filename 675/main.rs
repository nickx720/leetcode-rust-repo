impl Solution {
    pub fn flip_lights(n: i32, m: i32) -> i32 {
        let mut real_n = std::cmp::min(3, n);
    if m == 0 {
        return 1;
    }
    if m == 1 {
        real_n = if real_n == 1 {
            2
        } else if real_n == 2 {
            3
        } else {
            4
        };
        return real_n;
    }
    if m == 2 {
        real_n = if real_n == 1 {
            2
        } else if real_n == 2 {
            4
        } else {
            7
        };
        return real_n;
    }
    real_n = if real_n == 1 {
        2
    } else if real_n == 2 {
        4
    } else {
        8
    };
    return real_n;
    }
}