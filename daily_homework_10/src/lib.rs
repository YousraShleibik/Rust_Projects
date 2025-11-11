
// 1) smallest_word: first &str with the shortest length.
// Assumes the vector is non-empty (as the assignment implies).
pub fn smallest_word<'a>(v: Vec<&'a str>) -> &'a str {
    v.iter().copied().fold(v[0], |best, s| if s.len() < best.len() { s } else { best })
}

// 2) any_smaller: true if ANY tuple has BOTH components < n.
pub fn any_smaller(n: u32, v: &[(u32, u32)]) -> bool {
    v.iter().fold(false, |acc, &(a, b)| acc || (a < n && b < n))
}

// 3) first_some: first entry that is Some(..), else None.
// Uses into_iter() so we don't require T: Clone.
pub fn first_some<T>(v: Vec<Option<T>>) -> Option<T> {
    v.into_iter().fold(None, |acc, x| acc.or(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- smallest_word ----------
    #[test]
    fn sw_basic() {
        let words = vec!["alpha", "bee", "see", "d"];
        assert_eq!(smallest_word(words), "d");
    }

    #[test]
    fn sw_first_of_equal_lengths_wins() {
        let words = vec!["rust", "go", "js"];
        // "go" and "js" both length 2; first shortest ("go") should be chosen
        assert_eq!(smallest_word(words), "go");
    }

    #[test]
    fn sw_includes_empty_string() {
        let words = vec!["hello", "", "aaa", ""];
        // Empty string (len 0) should win; first empty is index 1
        assert_eq!(smallest_word(words), "");
    }

    #[test]
    fn sw_single_element() {
        let words = vec!["only"];
        assert_eq!(smallest_word(words), "only");
    }

    // NOTE: smallest_word() panics on empty Vec because it reads v[0].
    // If you need empty-safe behavior, we can return Option<&str> instead.

    // ---------- any_smaller ----------
    #[test]
    fn as_true_when_both_components_less_than_n() {
        let pairs = vec![(5, 7), (11, 3), (2, 1)];
        assert!(any_smaller(3, &pairs)); // (2,1) both < 3
    }

    #[test]
    fn as_false_when_only_one_component_is_less() {
        let pairs = vec![(4, 1), (1, 9)];
        assert!(!any_smaller(3, &pairs)); // never BOTH < 3
    }

    #[test]
    fn as_boundary_equals_are_not_less() {
        let pairs = vec![(3, 2), (2, 3), (3, 3)];
        // Comparison is strict (< n), so any 3s shouldn't pass
        assert!(!any_smaller(3, &pairs));
    }

    #[test]
    fn as_empty_vector_is_false() {
        let pairs: Vec<(u32, u32)> = vec![];
        assert!(!any_smaller(10, &pairs));
    }

    #[test]
    fn as_large_values() {
        let max = u32::MAX;
        let near = max - 1;
        let pairs = vec![(near, near), (max, 0)];
        // with n = max, near < max so (near, near) qualifies -> true
        assert!(any_smaller(max, &pairs));
        // with n = near, neither tuple has BOTH < near -> false
        assert!(!any_smaller(near, &pairs));
    }

    // ---------- first_some ----------
    #[test]
    fn fs_first_some_found() {
        let v = vec![None::<u32>, None, Some(7), Some(8)];
        assert_eq!(first_some(v), Some(7));
    }

    #[test]
    fn fs_some_is_first_element() {
        let v = vec![Some(99), None::<i32>, Some(1)];
        assert_eq!(first_some(v), Some(99));
    }

    #[test]
    fn fs_all_none() {
        let v: Vec<Option<i32>> = vec![None, None, None];
        assert_eq!(first_some(v), None);
    }

    #[test]
    fn fs_empty_vector() {
        let v: Vec<Option<&str>> = vec![];
        assert_eq!(first_some(v), None);
    }

    #[test]
    fn fs_works_without_clone_using_owned_type() {
        let v = vec![None, Some(String::from("hi")), Some(String::from("there"))];
        assert_eq!(first_some(v), Some("hi".to_string()));
    }

    #[test]
    fn fs_with_borrows() {
        // Even with &str, into_iter() gives Option<&str> by value (copy of pointer),
        // so this still works fine.
        let v = vec![None::<&str>, Some("a"), Some("b")];
        assert_eq!(first_some(v), Some("a"));
    }
}
