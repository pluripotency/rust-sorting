use crate::swap::swap;

// fn pivot<T: Ord + Copy>(array: &mut Vec<T>, i: usize, j: usize) -> T {
//     let mut mid = i + 1;
//     while mid < j {
//         if array[i] < array[mid] {
//             return array[mid]
//         } else if array[i] > array[mid] {
//             return array[i]
//         }
//         mid += 1;
//     }
//     array[j]
// }

fn pivot<T: PartialOrd + Copy>(array: &mut Vec<T>, i: usize, j: usize) -> T {
    let x = array[i];
    let y = array[i / 2 + j / 2]; // avoid (i+j:overflow)/2
    let z = array[j];
    if x < y {
        if y < z {
            y
        } else if z < x {
            x
        } else {
            z
        }
    } else {
        if z < y {
            y
        } else if x < z {
            x
        } else {
            z
        }
    }
}

fn partition<T: PartialOrd + Copy>(array: &mut Vec<T>, i: usize, j: usize, p: T) -> usize {
    let mut left = i;
    let mut right = j;
    while left <= right {
        while (left <= j) && (array[left] < p) {
            left += 1;
        }
        while (right >= i) && (array[right] > p) {
            right -= 1;
        }
        if left > right {
            break;
        }
        array.swap(left, right);
        left += 1;
        right -= 1;
    }
    left
}

pub fn quick_sort<T: PartialOrd + Copy>(array: &mut Vec<T>, i: usize, j: usize, iter_count: usize) {
    if i < j {
        let p = pivot(array, i, j);
        let k = partition(array, i, j, p);
        let iter_count = iter_count + 1;
        quick_sort(array, i, k - 1, iter_count);
        let iter_count = iter_count + 1;
        quick_sort(array, k, j, iter_count);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_quick_sort() {
//         let mut v = vec![20, 12, 45, 19, 91, 55];
//         let length = v.len();
//         quick_sort(&mut v, 0, length - 1, 1);
//         assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
//     }
//     #[test]
//     fn test_large_quick_sort() {
//         use rand::Rng;
//         let test_data: Vec<usize> = (0..100)
//             .map(|_| {
//                 rand::thread_rng().gen_range(1, 10000)
//                 // rand::thread_rng().gen_range(1, std::usize::MAX)
//             })
//             .collect();
//         let mut v = test_data.clone();
//         let length = v.len();
//         quick_sort(&mut v, 0, length - 1, 1);
//         println!("{:?}", v);
//     }
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quick_sort() {
        let test_data = vec![81.0, 20.0, 13.0, 32.0, 62.0, 54.0, 8.0, 95.0, 73.0, 19.0, 90.0, 23.0, 48.0, 6.0, 3.0];
        let test_answer = vec![3.0, 6.0, 8.0, 13.0, 19.0, 20.0, 23.0, 32.0, 48.0, 54.0, 62.0, 73.0, 81.0, 90.0, 95.0];
        let mut v = test_data.clone();
        quick_sort(&mut v, 0, test_data.len(), 1);
        assert_eq!(test_answer, v);
    }
}
