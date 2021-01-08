impl Solution {
    // Converting i32 to u32 wraps around the value and allows us to do 2s complement
    pub fn to_hex(num: i32) -> String {
       let mut num = num as u32;
    if num == 0 {
        return 0.to_string();
    }
    let mut hex: Vec<u32> = Vec::new();
    while num > 0 {
        let modu = num % 16;
        num = num / 16;
        hex.push(modu);
    }
    hex.reverse();
    let mut output = String::from("");
    for i in hex {
        match i {
            10 => output = format!("{}{}", output, "a"),
            11 => output = format!("{}{}", output, "b"),
            12 => output = format!("{}{}", output, "c"),
            13 => output = format!("{}{}", output, "d"),
            14 => output = format!("{}{}", output, "e"),
            15 => output = format!("{}{}", output, "f"),
            _ => output = format!("{}{}", output, i.to_string()),
        };
    }
    output 
    }
}