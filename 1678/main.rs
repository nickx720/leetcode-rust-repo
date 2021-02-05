impl Solution {
    pub fn interpret(command: String) -> String {
        let mut output: String = String::from("");
        let mut temp: Vec<char> = vec![];
        for chars in command.chars() {
            match chars {
                'G' => output += chars.to_string().as_str(),
                '(' => temp.push(chars),
                'a' => temp.push(chars),
                'l' => temp.push(chars),
                ')' => {
                    let s: String = temp.into_iter().collect();
                    if s == String::from("(al") {
                        output += "al";
                    } else {
                        output += "o";
                    }
                    temp = vec![]
                }
                _ => unreachable!(),
            }
        }
        output
    }
}
