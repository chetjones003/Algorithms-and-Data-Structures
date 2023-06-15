// Implementing a generic version
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        match item == target {
            true => return Some(index),
            false => (),
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::linear_search;

    #[test]
    fn search_ints() {
        assert_eq!(linear_search(&[1, 2, 3, 4], &1), Some(0));
        assert_eq!(linear_search(&[1, 2, 3, 4], &2), Some(1));
        assert_eq!(linear_search(&[1, 2, 3, 4], &3), Some(2));
        assert_eq!(linear_search(&[1, 2, 3, 4], &4), Some(3));
        assert_eq!(linear_search(&[1, 2, 3, 4], &5), None);
    }

    #[test]
    fn search_chars() {
        assert_eq!(linear_search(&['a', 'b', 'c', 'd'], &'a'), Some(0));
        assert_eq!(linear_search(&['a', 'b', 'c', 'd'], &'b'), Some(1));
        assert_eq!(linear_search(&['a', 'b', 'c', 'd'], &'c'), Some(2));
        assert_eq!(linear_search(&['a', 'b', 'c', 'd'], &'d'), Some(3));
        assert_eq!(linear_search(&['a', 'b', 'c', 'd'], &'e'), None);
    }

    #[test]
    fn search_strings() {
        assert_eq!(
            linear_search(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"Rust".to_string()
            ),
            Some(0)
        );
        assert_eq!(
            linear_search(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"is".to_string()
            ),
            Some(1)
        );
        assert_eq!(
            linear_search(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"Great".to_string()
            ),
            Some(2)
        );
        assert_eq!(
            linear_search(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"Not".to_string()
            ),
            None
        );
    }
}
