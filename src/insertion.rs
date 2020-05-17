pub fn insertion_sort<T, F>(array: &mut [T], comparator: &F)
    where
        T: Copy + PartialOrd,
        F: Fn(&T, &T) -> bool,
{
    for i in 1..array.len() {
        let num_to_insert = array[i];
        let mut index = i;
        while index > 0 && comparator(&num_to_insert, &array[index - 1]) {
            array[index] = array[index - 1];
            index -= 1;
        }
        array[index] = num_to_insert;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        let test_data = vec![81.0, 20.0, 13.0, 32.0, 62.0, 54.0, 8.0, 95.0, 73.0, 19.0, 90.0, 23.0, 48.0, 6.0, 3.0];
        let test_answer = vec![3.0, 6.0, 8.0, 13.0, 19.0, 20.0, 23.0, 32.0, 48.0, 54.0, 62.0, 73.0, 81.0, 90.0, 95.0];
        let mut v = test_data.clone();
        insertion_sort(&mut v, &|a, b| a < b);
        assert_eq!(test_answer, v);
    }
}
