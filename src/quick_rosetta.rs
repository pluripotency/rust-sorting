fn partition<T: Copy + PartialOrd>(v: &mut [T]) -> usize {
    let len = v.len();
    let pivot_index = len / 2;

    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in 0..len - 1 {
        if &v[i] < &v[len - 1] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

pub fn quick_sort<T: Copy + PartialOrd>(v: &mut [T]) {
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v);
        quick_sort(&mut v[0..pivot_index]);
        // quick_sort(&mut v[pivot_index + 1..len]);
        quick_sort(&mut v[pivot_index..len]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quick_sort() {
        let test_data = vec![81.0, 20.0, 13.0, 32.0, 62.0, 54.0, 8.0, 95.0, 73.0, 19.0, 90.0, 23.0, 48.0, 6.0, 3.0];
        let test_answer = vec![3.0, 6.0, 8.0, 13.0, 19.0, 20.0, 23.0, 32.0, 48.0, 54.0, 62.0, 73.0, 81.0, 90.0, 95.0];
        let mut v = test_data.clone();
        quick_sort(&mut v);
        assert_eq!(test_answer, v);
    }
}
