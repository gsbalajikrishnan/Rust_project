pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort(); // Sort the vector first
        nums.dedup(); // Remove duplicates
        nums.len() as i32 // Return the new length of the vector
        
    }
} 