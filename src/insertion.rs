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
        let mut v = vec![20, 12, 45, 19, 91, 55];
        insertion_sort(&mut v, &|a, b| a < b);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
    }
}
