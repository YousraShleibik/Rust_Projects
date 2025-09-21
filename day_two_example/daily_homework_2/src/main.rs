
// Daily Homework 2 Yousra Shleibik


// Problem 1: Multiply all elements of an array
// Function: multiply_array
// Parameters: arr - a slice of 32-bit unsigned integers (&[u32])
// Returns: a single 32-bit unsigned integer (u32) which is the product of all elements
fn multiply_array(arr: &[u32]) -> u32 {
    let mut result = 1;
    for &x in arr {
        result *= x;   
    }
    result
}

// Problem 2: Check if all numbers in an array are different
// Function: all_different
// Parameters: arr - a slice of 32-bit unsigned integers (&[u32])
// Returns: a boolean (true if all numbers are different, false if any duplicate exists)
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

// Problem 3: Check if all non-zero numbers in an array are different
// Function: all_different_except_zeros
// Parameters: arr - a slice of 32-bit unsigned integers (&[u32])
// Returns: a boolean (true if all non-zero numbers are unique, false if duplicates exist)
fn all_different_except_zeros(arr: &[u32]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] != 0 && arr[j] != 0 && arr[i] == arr[j] {
                println!("Duplicate non-zero found: {}", arr[i]);
                return false; // duplicate non-zero found
            }
        }
    }
    true
}


// Problem 4: Compute the dot product of two arrays
// Function: dot_product
// Parameters: 
//   a - first slice of 32-bit unsigned integers (&[u32])
//   b - second slice of 32-bit unsigned integers (&[u32])
// Returns: a single 32-bit unsigned integer (u32) which is the sum of pairwise products
// Panics if the two arrays are not the same length
fn dot_product(a: &[u32], b: &[u32]) -> u32 {
    if a.len() != b.len() {
        panic!("PANIC! Arrays must be the same length!");
    }

    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i] * b[i]; // multiply pair and add
    }
    sum
}


fn main() {
     
    let arr1 = [2, 3, 4];
    let arr2 = [6, 6, 6, 0, 0];
    let arr3 = [0, 1, 2, 3, 0];

    // Multiply all numbers in arr1 → 2*3*4 = 24 
    println!("multiply_array(arr1) = {}", multiply_array(&arr1));

    // Check if all numbers in arr1 are different → true
    println!("all_different(arr2) = {}", all_different(&arr2));

    // Check if all numbers in arr2 are different except zeros → true
    println!("all_different_except_zeros(arr3) = {}", all_different_except_zeros(&arr3));

    // Dot product of arr2 and arr3
    println!("dot_product(arr2, arr3) = {}", dot_product(&arr2, &arr3));

}
