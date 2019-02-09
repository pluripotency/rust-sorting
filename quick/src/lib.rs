use swap::swap;

fn pivot<T: Ord + Copy>(array: &mut Vec<T>, i: usize, j: usize) -> T {
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

fn partition<T: Ord + Copy>(array: &mut Vec<T>, i: usize, j: usize, p: T) -> usize {
    let mut l = i.clone();
    let mut r = j.clone();
    loop {
        while array[l] < p {
            l += 1;
        }
        while p < array[r] {
            r -= 1;
        }
        if l > r {
            break;
        }
        swap(array, l, r);
        l += 1;
        r -= 1;
    }
    l
}

pub fn quick_sort<T: Ord + Copy>(array: &mut Vec<T>, i: usize, j: usize) {
    if i < j {
        let p = pivot(array, i, j);
        let k = partition(array, i, j, p);
        quick_sort(array, i, k - 1);
        quick_sort(array, k, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![20, 12, 45, 19, 91, 55];
        let length = v.len();
        quick_sort(&mut v, 0, length - 1);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
    }
}
