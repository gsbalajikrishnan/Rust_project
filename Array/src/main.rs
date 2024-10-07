// Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message

//mod duplicate_array;
mod twosum;
//use duplicate_array::Solution; 
use twosum::TwoSum;

fn main() {

    /* // Removing duplicate array 
    let mut nums = vec![1, 2, 2, 3, 4, 4, 5];
    let var = 2;
    let new_length = Solution::remove_duplicates(&mut nums, var);
    println!("length of array: {}", new_length);
    println!("Updated vector: {:?}", nums);  */
    // Two Sum Array
    let arrsum = vec![3,2,4];
    let target = 6;
    let ind = TwoSum::two_sum(arrsum, target);
    println!("Sum of two array index [{:?},{:?}]", ind[0], ind[1]);
}