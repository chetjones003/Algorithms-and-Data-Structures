pub fn binary_search(arr: &[usize], target: &usize) -> Option<usize> {}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn basic_tests() {
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &1), Some(1));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &2), Some(2));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &3), Some(3));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &4), Some(4));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &5), Some(5));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &6), Some(6));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &7), Some(7));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &8), Some(8));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &9), Some(9));
        assert_eq!(binary_search(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &0), Some(0));
    }
}
