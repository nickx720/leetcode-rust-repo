impl Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() || candidates.len() == 0 {
            vec![]
        } else {
            let mut results: Vec<Vec<i32>> = Vec::new();
            let mut sorted_candidate: Vec<i32> = candidates;
            sorted_candidate.sort();
            let mut combination: Vec<i32> = Vec::new();
            find_non_repeating_sum(&mut results, &mut combination, &sorted_candidate, target, 0);
            results
        }
    }

    fn find_non_repeating_sum(
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
            // chceks if it is duplicate or not already existing
            if i != (start_index as usize) && candidates[i] == candidates[i - 1] {
                continue;
            }
            if candidates[i] > target {
                break;
            }
            combination.push(candidates[i]);
            find_non_repeating_sum(
                results,
                combination,
                candidates,
                target - candidates[i],
                // increment i for the next series
                i as i32 + 1,
            );
            combination.pop();
        }
    }
}
