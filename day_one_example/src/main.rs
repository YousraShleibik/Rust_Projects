

// fn compute(a: u32, b: u32) -> u32 {
//     a + b * 2
// }


// #[cfg(test)]
// mod tests {
//     use crate::compute;
//
//     #[test]
//     fn case() {
//         assert_eq!(compute(1, 2), 5);
//     }
//
//     #[test]
//     fn case_two() {
//         assert_eq!(compute(0, 0), 0);
//     }
// }


use day_one_example::{quadratic, scale_vector, dot_product};

fn main() {
    println!("Hello, world!");

    println!("quadratic(1, 2, 3, 4) = {}", quadratic(1, 2, 3, 4));
    println!("scale_vector(5.0, (3.0, 4.0)) = {:?}", scale_vector(5.0, (3.0, 4.0)));
    println!("dot_product((2.0, 5.0), (7.0, 1.0)) = {}", dot_product((2.0, 5.0), (7.0, 1.0)));

}