// Problem 1: Count how many times a given character appears in an array
// Function: count_occurrences
// Parameters: 
//   ch  - a single character (char)
//   arr - a slice of characters (&[char])
// Returns: a 32-bit unsigned integer (u32) = the number of times `ch` occurs in `arr`

fn count_occurrences(ch: char, arr: &[char]) -> u32 {
    let mut count = 0;
    for i in 0..arr.len() {
        if arr[i] == ch {
            count += 1;
        }
    }
    count
}


// Problem 2: Square each element of an array
// Function: square_elements
// Parameters: 
//   arr - a mutable slice of 32-bit unsigned integers (&mut [u32])
// Returns: nothing (the array is modified in place)
fn square_elements(arr: &mut [u32]) {
    for i in 0..arr.len() {
        arr[i] = arr[i] * arr[i]; // replace each element with its square
    }
}

fn main() {
      // Problem 1 demo
    let chars = ['b', 'b', 'a', 'c', 'a'];
    println!("count_occurrences('b') = {}", count_occurrences('b', &chars));

    // Problem 2 demo
    let mut nums = [1, 3, 5];
    square_elements(&mut nums);
    println!("square_elements of [1, 3, 5] = {:?}", nums);
   
}
