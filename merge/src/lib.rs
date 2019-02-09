fn merge(array: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
    let mut i = left;
    let mut j = mid + 1;
    let mut v: Vec<usize> = vec![];

    while (i <= mid) && (j <= right) {
        i = left;
        j = mid + 1;
        while (i <= mid) && (j <= right) {
            print!("{} ? {}  ", array[i], array[j]);
            if array[i] <= array[j] {
                println!("smaller is {}", array[i]);
                v.push(array[i]);
                i += 1;
            } else {
                println!("smaller is {}", array[j]);
                v.push(array[j]);
                j += 1;
            }
        }
        if i == mid + 1 {
            print!("after mid ");
            while j <= right {
                print!("append {} ", array[j]);
                v.push(array[j]);
                j += 1;
            }
        } else {
            print!("before mid ");
            while i <= mid {
                print!("append {} ", array[i]);
                v.push(array[i]);
                i += 1;
            }
        }
    }
    for k in 0..v.len() {
        array[k + left] = v[k];
    }
    println!("\ncurrently: {:?}", array);
}

pub fn merge_sort(array: &mut Vec<usize>, left: usize, right: usize) {
    if left == right {
        return;
    }
    let mid = left / 2 + right / 2;
    merge_sort(array, left, mid);
    merge_sort(array, mid + 1, right);
    merge(array, left, mid, right);
}

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
        let mut v = test_data.clone();
        let length = v.len();
        merge_sort(&mut v, 0, length - 1);
        assert_eq!(test_answer, v);
    }
}
