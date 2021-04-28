impl Solution {
    pub fn simplify_path(path: String) -> String {
         use std::collections::VecDeque;
    let mut path_stack: VecDeque<&str> = VecDeque::new();
    let path = path
        .trim()
        .split("/")
        .filter(|&x| x != "")
        .collect::<Vec<&str>>();
    for item in path {
        match item {
            "" | "." => continue,
            ".." => {
                let _ = path_stack.pop_back();
            }
            val => path_stack.push_back(val),
            _ => unreachable!(),
        }
    }
    let output = path_stack.iter().map(|&x| x).collect::<Vec<&str>>();
    let output = "/".to_string() + &output.join("/");
    output
    }
}