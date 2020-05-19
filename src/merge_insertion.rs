use super::insertion_native::insert_head;

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

pub fn merge_sort<T: Copy + PartialOrd>(x: &mut [T]) {
    const MAX_INSERTION: usize = 20;
    let len = x.len();
    if len <= 1 {
        return;
    }
    if len <= MAX_INSERTION {
        if len >= 2 {
            for i in (0..len - 1).rev() {
                insert_head(&mut x[i..], &mut |a, b| a.lt(b));
            }
        }
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
        let test_data = vec![81, 20, 13, 32, 62, 54, 8, 95, 73, 19, 90, 23, 48, 6, 3];
        let test_answer = vec![3, 6, 8, 13, 19, 20, 23, 32, 48, 54, 62, 73, 81, 90, 95];
        let mut v = test_data.clone();
        merge_sort(&mut v);
        assert_eq!(test_answer, v);
    }
}
