fn append_rest<T: Copy + PartialOrd>(v: &mut Vec<T>, array: &[T], from: usize, to: usize) -> usize {
    for i in from..(to + 1) {
        v.push(array[i]);
    }
    to
}
fn merge<T: Copy + PartialOrd>(array: &mut [T], left: usize, mid: usize, right: usize) {
    let mut i = left;
    let mut j = mid + 1;
    let mut v: Vec<T> = vec![];

    while (i <= mid) && (j <= right) {
        i = left;
        j = mid + 1;
        while (i <= mid) && (j <= right) {
            if array[i] < array[j] {
                v.push(array[i]);
                i += 1;
            } else {
                v.push(array[j]);
                j += 1
            }
        }
        if i == mid + 1 {
            j = append_rest(&mut v, &array, j, right);
        } else {
            i = append_rest(&mut v, &array, i, mid);
        }
    }
    for k in 0..v.len() {
        array[k + left] = v[k];
    }
}
pub fn merge_sort<T: Copy + PartialOrd>(array: &mut [T], left: usize, right: usize) {
    if left == right {
        return;
    }
    let mid = left / 2 + right / 2;
    merge_sort(array, left, mid);
    merge_sort(array, mid + 1, right);
    merge(array, left, mid, right);
}

//fn merge(array: &mut [usize], left: usize, mid: usize, right: usize) {
//    let mut i = left;
//    let mut j = mid + 1;
//    let mut k = left;
//
//    while (i <= mid) && (j <= right) {
//        let tmp = array[k];
//        if array[i] < array[j] {
//            println!("i is smaller {:?}", array[i]);
//            println!("k {:?}", array[k]);
//            array[k] = array[i];
//            i += 1;
//            k += 1;
//        } else {
//            println!("j is smaller {:?}", array[j]);
//            println!("k {:?}", array[k]);
//            array[k] = array[j];
//            j += 1;
//            k += 1;
//        }
//    }
//}
//
//pub fn merge_sort(array: &mut [usize], left: usize, right: usize) {
//    if left == right {
//        return;
//    }
//    let mid = left / 2 + right / 2;
//    merge_sort(array, left, mid);
//    merge_sort(array, mid + 1, right);
//    merge(array, left, mid, right);
//}

#[cfg(test)]
mod tests {
    use super::*;
    //    #[test]
    //    fn test_merge_sort_small() {
    //        let test_data = vec![81, 20, 13, 32, 62, 54, 8];
    //        let test_answer = vec![8, 13, 20, 32, 54, 62, 81];
    //        let mut v = test_data.clone();
    //        let length = v.len();
    //        merge_sort(&mut v, 0, length - 1);
    //        assert_eq!(test_answer, v);
    //    }
    #[test]
    fn test_merge_sort() {
        let test_data = vec![81, 20, 13, 32, 62, 54, 8, 95, 73, 19, 90, 23, 48, 6, 3];
        let test_answer = vec![3, 6, 8, 13, 19, 20, 23, 32, 48, 54, 62, 73, 81, 90, 95];
        //        let test_data = vec![90, 23, 48, 6, 3];
        //        let test_answer = vec![3, 6, 23, 48, 90];
        let mut v = test_data.clone();
        let length = v.len();
        merge_sort(&mut v, 0, length - 1);
        assert_eq!(test_answer, v);
    }
}
