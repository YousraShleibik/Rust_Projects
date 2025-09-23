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

// Problem 3: Apply a given function to each element of an array
// Function: map_elements
// Parameters: 
//   f   - a function pointer (fn(u32) -> u32)
//   arr - a mutable slice of 32-bit unsigned integers (&mut [u32])
// Returns: nothing (the array is modified in place)
fn map_elements(f: fn(u32) -> u32, arr: &mut [u32]) {
    for i in 0..arr.len() {
        arr[i] = f(arr[i]); // apply the function to each element
    }
}

// Problem 4: Count the number of true and false values in an array
// Function: count_true_and_false
// Parameters: 
//   arr - a slice of boolean values (&[bool])
// Returns: a tuple (u32, u32)
//   First element = number of true values
//   Second element = number of false values
fn count_true_and_false(arr: &[bool]) -> (u32, u32) {
    let mut true_count = 0;
    let mut false_count = 0;
    for i in 0..arr.len() {
        if arr[i] {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }
    (true_count, false_count)
}

fn main() {
      // Problem 1 demo
    let chars = ['b', 'b', 'a', 'c', 'a'];
    println!("count_occurrences('b') = {}", count_occurrences('b', &chars));

    // Problem 2 demo
    let mut nums = [1, 3, 5];
    square_elements(&mut nums);
    println!("square_elements of [1, 3, 5] = {:?}", nums);


        // Problem 3 demo
    fn subtract_one(x: u32) -> u32 { x - 1 }
    let mut nums2 = [1, 3, 5];
    map_elements(subtract_one, &mut nums2);
    println!("map_elements(subtract_one) = {:?}", nums2);

     // Problem 4 demo
    let bools = [true, false, true, true, false];
    let (t, f) = count_true_and_false(&bools);
    println!("count_true_and_false = (true={}, false={})", t, f);
   
}


#[cfg(test)]
mod tests {
    use super::*;

    // --- count_occurrences tests ---
    #[test]
    fn count_occurrences_basic() {
        let chars = ['a', 'b', 'a', 'c'];
        assert_eq!(count_occurrences('a', &chars), 2);
    }

    #[test]
    fn count_occurrences_none() {
        let chars = ['x', 'y', 'z'];
        assert_eq!(count_occurrences('a', &chars), 0);
    }

    #[test]
    fn count_occurrences_all_same() {
        let chars = ['m', 'm', 'm'];
        assert_eq!(count_occurrences('m', &chars), 3);
    }

    // --- square_elements tests ---
    #[test]
    fn square_elements_basic() {
        let mut nums = [1, 2, 3];
        square_elements(&mut nums);
        assert_eq!(nums, [1, 4, 9]);
    }

    #[test]
    fn square_elements_with_zero() {
        let mut nums = [0, 5, 10];
        square_elements(&mut nums);
        assert_eq!(nums, [0, 25, 100]);
    }

    #[test]
    fn square_elements_empty() {
        let mut nums: [u32; 0] = [];
        square_elements(&mut nums);
        assert_eq!(nums, []);
    }

    // --- map_elements tests ---
    fn add_one(x: u32) -> u32 { x + 1 }
    fn double(x: u32) -> u32 { x * 2 }

    #[test]
    fn map_elements_add_one() {
        let mut nums = [1, 2, 3];
        map_elements(add_one, &mut nums);
        assert_eq!(nums, [2, 3, 4]);
    }

    #[test]
    fn map_elements_double() {
        let mut nums = [2, 4, 6];
        map_elements(double, &mut nums);
        assert_eq!(nums, [4, 8, 12]);
    }

    #[test]
    fn map_elements_empty() {
        let mut nums: [u32; 0] = [];
        map_elements(add_one, &mut nums);
        assert_eq!(nums, []);
    }

    // --- count_true_and_false tests ---
    #[test]
    fn count_true_and_false_mixed() {
        let arr = [true, false, true];
        assert_eq!(count_true_and_false(&arr), (2, 1));
    }

    #[test]
    fn count_true_and_false_all_true() {
        let arr = [true, true, true];
        assert_eq!(count_true_and_false(&arr), (3, 0));
    }

    #[test]
    fn count_true_and_false_all_false() {
        let arr = [false, false];
        assert_eq!(count_true_and_false(&arr), (0, 2));
    }
}