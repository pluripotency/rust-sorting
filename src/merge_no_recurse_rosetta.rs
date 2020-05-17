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
    let n = x.len();
    let mut y = x.to_vec();
    let mut len = 1;
    while len < n {
        let mut i = 0;
        while i < n {
            if i + len >= n {
                y[i..].copy_from_slice(&x[i..]);
            } else if i + 2 * len > n {
                merge(&x[i..i + len], &x[i + len..], &mut y[i..]);
            } else {
                merge(
                    &x[i..i + len],
                    &x[i + len..i + 2 * len],
                    &mut y[i..i + 2 * len],
                );
            }
            i += 2 * len;
        }
        len *= 2;
        if len >= n {
            x.copy_from_slice(&y);
            return;
        }
        i = 0;
        while i < n {
            if i + len >= n {
                x[i..].copy_from_slice(&y[i..]);
            } else if i + 2 * len > n {
                merge(&y[i..i + len], &y[i + len..], &mut x[i..]);
            } else {
                merge(
                    &y[i..i + len],
                    &y[i + len..i + 2 * len],
                    &mut x[i..i + 2 * len],
                );
            }
            i += 2 * len;
        }
        len *= 2;
    }
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
