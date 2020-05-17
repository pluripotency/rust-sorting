fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }else if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn merge_sort<T: Copy + PartialOrd>(x: &mut [T]) {
    let len = x.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;

    merge_sort(&mut x[0..mid]);
    merge_sort(&mut x[mid..len]);

    let mut y: Vec<T> = x.to_vec();

    merge(&x[0..mid], &x[mid..len], &mut y[..]);

    x.copy_from_slice(&y);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let test_data = vec![81.0, 20.0, 13.0, 32.0, 62.0, 54.0, 8.0, 95.0, 73.0, 19.0, 90.0, 23.0, 48.0, 6.0, 3.0];
        let test_answer = vec![3.0, 6.0, 8.0, 13.0, 19.0, 20.0, 23.0, 32.0, 48.0, 54.0, 62.0, 73.0, 81.0, 90.0, 95.0];
        let mut v = test_data.clone();
        merge_sort(&mut v);
        assert_eq!(test_answer, v);
    }
}
