
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


fn main() {
     
    let arr1 = [2, 3, 4];
    //let arr2 = [5, 6, 7];

    // Multiply all numbers in arr1 → 2*3*4 = 24 
    println!("multiply_array(arr1) = {}", multiply_array(&arr1));
    
}
