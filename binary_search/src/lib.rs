pub fn binary_search(arr: &[i64], target: &i64) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;
    while low <= high {
        let midpoint: usize = low + ((high - low) / 2);
        let value: &i64 = &arr[midpoint];
        if value == target {
            return Some(midpoint);
        } else if value > target {
            high = midpoint - 1
        } else if value < target {
            low = midpoint + 1
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn basic_tests() {
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &1), Some(0));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &2), Some(1));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &3), Some(2));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &4), Some(3));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &5), Some(4));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &6), Some(5));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &7), Some(6));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &8), Some(7));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &9), Some(8));
        assert_eq!(
            binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &10),
            Some(9)
        );
    }
}
