pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}