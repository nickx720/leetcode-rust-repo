impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        fn nums_push(res: &mut Vec<Vec<i32>>, repo: Vec<i32>, items: Vec<i32>) {
            if items.len() == 0 {
                res.push(repo.clone());
                return;
            }
            for (id, &stuff) in items.iter().enumerate() {
                // remove the index of the elment which has been iterated over
                let mut removedId = items.clone();
                removedId.remove(id);
                let mut newRepo = repo.clone();
                // push the completed permutation into array
                newRepo.push(stuff);
                nums_push(res, newRepo, removedId)
            }
        }
        if nums.len() != 0 {
            nums_push(&mut res, Vec::new(), nums);
        }
        res
    }
}
