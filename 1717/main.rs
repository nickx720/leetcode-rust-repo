impl Solution {
    // https://dev.to/seanpgallivan/solution-maximum-score-from-removing-substrings-ver-1-39g2
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
      let (first,second,high,low) = if x > y {
        ('a','b',x,y)
    } else {
        ('b','a',y,x)
    };
    let mut stack_1:Vec<char> = vec![];
    let mut stack_2:Vec<char> = vec![];
    let mut score = 0i32;
    for ch in s.chars(){
        if stack_1.len() >0 && second == ch && stack_1[stack_1.len()-1] == first {
            score+=high;
            stack_1.pop();
        }else{
            stack_1.push(ch);
        }
    }
    while stack_1.len() > 0 {
        let ch = stack_1.pop().unwrap();
        if stack_2.len()>0 && second == ch && stack_2[stack_2.len()-1] == first {
            score+=low;
            stack_2.pop();
        } else {
            stack_2.push(ch);
        }
    }
    score
    }
}