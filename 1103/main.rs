impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let num_people = num_people as usize;
        let mut output:Vec<i32> = vec![0;num_people];
        let (mut candies, mut i) = (candies,0);
        while candies > 0 {
            let position = i % num_people;
            let increment = i as i32;
            output[position] += std::cmp::min(candies,increment+1);
            candies -= increment +1;
            i+=1;
        }
        output

    }
}
