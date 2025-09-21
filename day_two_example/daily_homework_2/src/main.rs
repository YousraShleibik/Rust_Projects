
//use std::collections::HashSet;

//multiply_array
fn multiply_array(arr: &[u32]) -> u32 {
    let mut result = 1;
    for &x in arr {
        result *= x;   
    }
    result
}

//all_different
fn all_different(arr: &[u32]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                println!("Duplicate found: {}", arr[i]);
                return false; // found a duplicate
                
            }
        }
    }
    true // no duplicates found
    
}

fn main() {
     
    let arr1 = [2, 3, 4];
    let arr2 = [6, 6, 6];

    // Multiply all numbers in arr1 → 2*3*4 = 24 
    println!("multiply_array(arr1) = {}", multiply_array(&arr1));

    // Check if all numbers in arr1 are different → true
     println!("all_different(arr1) = {}", all_different(&arr2));

    
}
