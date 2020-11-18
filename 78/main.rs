impl Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output = Vec::new();
        backtrackSearch(&nums[..], 0, &mut vec![], &mut output);
        output
    }

    fn backtrackSearch(nums: &[i32], id: usize, cur: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
        output.push(cur.clone());
        (id..nums.len()).for_each(|x| {
            cur.push(nums[x]); //push each item into the current stack and pop, also increment the x, so it traverses for each
            backtrackSearch(nums, x + 1, cur, output);
            cur.pop();
        })
    }
}
