pub fn add_one(v: &mut Vec<u32>) {
    for x in v.iter_mut() {
        *x += 1;
    }
}


pub fn get_total(v: &Vec<u32>) -> u32 {
    v.iter().copied().sum()
}


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