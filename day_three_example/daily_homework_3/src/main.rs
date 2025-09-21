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

fn main() {
      // Problem 1 demo
    let chars = ['b', 'b', 'a', 'c', 'a'];
    println!("count_occurrences('b') = {}", count_occurrences('b', &chars));
   
}
