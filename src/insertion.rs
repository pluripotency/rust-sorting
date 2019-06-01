pub fn insertion_sort<T: Copy + PartialOrd, F: Fn(&T, &T) -> bool>(
    array: &mut [T],
    comparator: &F,
) {
    for i in 1..array.len() {
        let num_to_insert = array[i];
        let mut compare_index = i;
        while compare_index > 0 && comparator(&num_to_insert, &array[compare_index - 1]) {
            array[compare_index] = array[compare_index - 1];
            compare_index -= 1;
        }
        array[compare_index] = num_to_insert;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![20, 12, 45, 19, 91, 55];
        insertion_sort(&mut v, &|a, b| a < b);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
    }
}
