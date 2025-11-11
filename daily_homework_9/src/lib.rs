

/// Returns all values in `v` that are smaller than `n`.
pub fn smaller_than(n: u32, v: &[u32]) -> Vec<u32> {
    v.iter().copied().filter(|&x| x < n).collect()
}

/// Returns all &str values from tuples whose key == `key`.
pub fn get_values<'a>(key: u32, v: &'a [(u32, &'a str)]) -> Vec<&'a str> {
    v.iter().filter(|(k, _)| *k == key).map(|(_, s)| *s).collect()
}

/// Drops `None` and unwraps `Some(T)` into a Vec<T>.
/// (Takes ownership of the vector so we can move T out without cloning.)
pub fn only_some<T>(v: Vec<Option<T>>) -> Vec<T> {
    v.into_iter().filter(|o| o.is_some()).map(|o| o.unwrap()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_than_basic() {
        let nums = vec![1, 5, 2, 9, 3, 7];
        assert_eq!(smaller_than(5, &nums), vec![1, 2, 3]);
    }

    #[test]
    fn test_smaller_than_edge_cases() {
        let empty: Vec<u32> = vec![];
        assert_eq!(smaller_than(10, &empty), vec![]);

        let all_large = vec![10, 10, 11];
        assert_eq!(smaller_than(10, &all_large), vec![]);

        let all_small = vec![0, 1, 2, 3];
        assert_eq!(smaller_than(10, &all_small), all_small);
    }

    #[test]
    fn test_get_values_basic() {
        let pairs: Vec<(u32, &str)> = vec![(1, "a"), (2, "b"), (1, "c"), (3, "d")];
        assert_eq!(get_values(1, &pairs), vec!["a", "c"]);
        assert_eq!(get_values(2, &pairs), vec!["b"]);
        assert_eq!(get_values(4, &pairs), Vec::<&str>::new());
    }

    #[test]
    fn test_only_some_basic() {
        let data = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(only_some(data), vec![1, 3, 5]);
    }

    #[test]
    fn test_only_some_strings() {
        let data = vec![
            Some(String::from("hi")),
            None,
            Some(String::from("there")),
        ];
        assert_eq!(only_some(data), vec!["hi".to_string(), "there".to_string()]);
    }

    #[test]
    fn test_only_some_empty() {
        let data: Vec<Option<u32>> = vec![];
        let out = only_some(data);
        assert!(out.is_empty());
    }
}

