impl Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        if candidates.is_empty() || candidates.len() == 0 {
            vec![]
        } else {
            let mut sorted_candidates: Vec<i32> = candidates;
            sorted_candidates.sort();
            let mut combination: Vec<i32> = Vec::new();
            fn_to_dfs_and_find_combination(
                &mut results,
                &mut combination,
                &sorted_candidates,
                target,
                0,
            );
            results
        }
    }
    // we are splitting the problem, so subtract the target with the current candidate,and recursively search till current value is greater than candidate
    // look at youtube for clearer description
    fn fn_to_dfs_and_find_combination(
        results: &mut Vec<Vec<i32>>,
        combination: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        start_index: i32,
    ) {
        if target == 0 {
            results.push(combination.to_vec())
        }

        for i in (start_index as usize)..candidates.len() {
            if candidates[i] > target {
                break;
            }
            combination.push(candidates[i]);
            fn_to_dfs_and_find_combination(
                results,
                combination,
                candidates,
                target - candidates[i],
                i as i32,
            );
            combination.pop();
        }
    }
}
