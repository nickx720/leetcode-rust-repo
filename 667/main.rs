/// https://dev.to/seanpgallivan/solution-beautiful-arrangement-ii-1lag
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut answer = vec![0;n as usize];
        let (mut a, mut z) = (1,k+1);
        for i in 0..=k{
            if i%2 ==0 {
                answer[i as usize] = a;
                a+=1;
            } else {
                answer[i as usize] = z;
                z-=1;
            }
        }
        let k = k +1;
        for i in k..n{
            answer[i as usize] = i+1;
        }
        answer

    }
}
