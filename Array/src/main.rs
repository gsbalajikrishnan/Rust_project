// Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message

mod duplicate_array;

use duplicate_array::Solution;

fn main() {
    let mut nums = vec![1, 2, 2, 3, 4, 4, 5];
    let var = 2;
    let new_length = Solution::remove_duplicates(&mut nums, var);
    println!("length of array: {}", new_length);
    println!("Updated vector: {:?}", nums);
}