pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut swapped: bool = false;
    for i in 0..(arr.len() - 1) {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);
            swapped = true;
        }
    }

    if swapped {
        bubble_sort(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn basic_tests() {
        let mut arr: Vec<i32> = vec![1, 3, 4, 7, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 7])
    }
}
