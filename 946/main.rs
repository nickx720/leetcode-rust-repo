impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pushed = pushed;
        let (length,mut i,mut j,mut s) = (pushed.len(),0,0,0);
        while i < length{
            if s >=0 && popped[j] == pushed[s as usize]{
                j+=1;
                s-=1;
            } else {
                s+=1;
                i+=1;
                if i < length{
                    pushed[s as usize] = pushed[i];
                }
            }
        }
        s==0

    }
}
