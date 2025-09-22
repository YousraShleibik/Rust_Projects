
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
    let arr2 = [6, 3, 2, 0, 0];
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- multiply_array tests ---
    #[test]
    fn multiply_basic() {
        assert_eq!(multiply_array(&[2, 3, 4]), 24);
    }

    #[test]
    fn multiply_with_zero() {
        assert_eq!(multiply_array(&[5, 0, 7]), 0);
    }

    #[test]
    fn multiply_empty_is_one() {
        // By convention here, product of empty slice = 1 (neutral element)
        let empty: [u32; 0] = [];
        assert_eq!(multiply_array(&empty), 1);
    }


    // --- Check all_different tests ---
    #[test]
    fn all_diff_true_when_unique() {
        assert!(all_different(&[1, 2, 3, 4]));
    }

    #[test]
    fn all_diff_false_when_duplicate_present() {
        assert!(!all_different(&[1, 2, 2, 3]));
    }

    #[test]
    fn all_diff_handles_zeros_like_any_value() {
        assert!(!all_different(&[0, 1, 0]));
    }

    // --- all_different_except_zeros tests ---
    #[test]
    fn all_diff_except_zeros_true_when_only_zeros_repeat() {
        assert!(all_different_except_zeros(&[0, 1, 2, 0, 3]));
    }

    #[test]
    fn all_diff_except_zeros_false_when_nonzero_repeats() {
        assert!(!all_different_except_zeros(&[0, 2, 2, 0]));
    }

     // --- dot_product tests ---
    #[test]
    fn dot_product_basic() {
        // (1*4) + (2*5) + (3*6) = 32
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
    }

    #[test]
    fn dot_product_with_zeros() {
        assert_eq!(dot_product(&[0, 10], &[99, 1]), 10);
    }
  #[test]
    fn dot_product_empty_is_zero() {
        let a: [u32; 0] = [];
        let b: [u32; 0] = [];
        assert_eq!(dot_product(&a, &b), 0);
    }

    #[test]
    #[should_panic(expected = "Arrays must be the same length")]
    fn dot_product_unequal_lengths_panics() {
        let a = [1, 2];
        let b = [1];
        let _ = dot_product(&a, &b);
    }
}
