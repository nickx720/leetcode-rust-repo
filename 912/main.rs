impl Solution {
    // steps to do merge sort
    // split the array at half
    // call sort fn on left half and right half
    // create new array and push each element in left and right into results.
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() > 1 {
            let (l, r) = nums.split_at(nums.len() / 2);
            let sorted_l = Self::sort_array(l.to_vec());
            let sorted_r = Self::sort_array(r.to_vec());
            let mut result: Vec<i32> = nums.into();
            let (mut i, mut j, mut k) = (0, 0, 0);
            while i < sorted_l.len() && j < sorted_r.len() {
                if sorted_l[i] <= sorted_r[j] {
                    result[k] = sorted_l[i];
                    i += 1;
                } else {
                    result[k] = sorted_r[j];
                    j += 1;
                }
                k += 1;
            }
            while i < sorted_l.len() {
                result[k] = sorted_l[i];
                k += 1;
                i += 1;
            }
            while j < sorted_r.len() {
                result[k] = sorted_r[j];
                k += 1;
                j += 1;
            }
            result
        } else {
            nums
        }
    }
}
