
/// Add one to every element in the given vector **in place**.
/// # Parameters
/// - `v`: `&mut Vec<u32>` — mutable reference to a vector of 32-bit unsigned integers.
/// # Returns
/// - `()` — no return value; the input vector is modified directly.
///
pub fn add_one(v: &mut Vec<u32>) {
    for x in v.iter_mut() {
        *x += 1;
    }
}


/// Compute the total (sum) of all elements in the given vector using an iterator.
/// # Parameters
/// - `v`: `&Vec<u32>` — reference to a vector of 32-bit unsigned integers.
/// # Returns
/// - `u32` — sum of all elements (wraps on `u32` overflow as per Rust semantics).

pub fn get_total(v: &Vec<u32>) -> u32 {
    v.iter().copied().sum()
}

/// Convert a vector of `(u32, u32)` pairs into a vector of their pairwise sums.
/// Implemented in **one line** using `into_iter()` + `map()` + `collect()`.
/// # Parameters
/// - `pairs`: `Vec<(u32, u32)>` — vector of 32-bit unsigned integer tuples.
/// # Returns
/// - `Vec<u32>` — vector where each element is `a + b` for `(a, b)` in `pairs`.
pub fn sum_tuple(pairs: Vec<(u32, u32)>) -> Vec<u32> {
    pairs.into_iter().map(|(a, b)| a + b).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one_empty() {
        let mut v: Vec<u32> = vec![];
        add_one(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn test_add_one_basic() {
        let mut v = vec![0, 1, 2, 42, u32::MAX - 1];
        add_one(&mut v);
        // Note: u32::MAX - 1 + 1 == u32::MAX; no overflow in these cases.
        assert_eq!(v, vec![1, 2, 3, 43, u32::MAX]);
    }

    #[test]
    fn test_get_total_empty() {
        let v: Vec<u32> = vec![];
        assert_eq!(get_total(&v), 0);
    }

    #[test]
    fn test_get_total_basic() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(get_total(&v), 10);
    }

    #[test]
    fn test_sum_tuple_basic() {
        let pairs = vec![(1, 2), (10, 5), (0, 7)];
        let out = sum_tuple(pairs);
        assert_eq!(out, vec![3, 15, 7]);
    }

    #[test]
    fn test_sum_tuple_large() {
        let pairs = vec![(u32::MAX - 1, 1), (100, 200)];
        let out = sum_tuple(pairs);
        assert_eq!(out, vec![u32::MAX, 300]);
    }
}