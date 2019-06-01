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
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let n = x.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut x[0..m]);
    merge_sort(&mut x[m..n]);

    let mut y: Vec<T> = x.to_vec();

    merge(&x[0..m], &x[m..n], &mut y[..]);

    x.copy_from_slice(&y);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let test_data = vec![81, 20, 13, 32, 62, 54, 8, 95, 73, 19, 90, 23, 48, 6, 3];
        let test_answer = vec![3, 6, 8, 13, 19, 20, 23, 32, 48, 54, 62, 73, 81, 90, 95];
        let mut v = test_data.clone();
        merge_sort(&mut v);
        assert_eq!(test_answer, v);
    }
}
