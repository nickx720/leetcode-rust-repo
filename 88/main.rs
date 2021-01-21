impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut first = m as isize - 1;
    let mut second = n as isize - 1;
    let length = (m + n) as usize;
    for i in (0..length).rev() {
        if second < 0 {
            break;
        }

        if first >= 0 && nums1[first as usize] > nums2[second as usize] {
            //    swap empty position with element at first position in nums1
            nums1[i] = nums1[first as usize];
            first -= 1;
        } else {
            nums1[i] = nums2[second as usize];
            second -= 1;
        }
    } 
    }
}