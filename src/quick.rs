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
        quick_sort(&mut v[pivot_index + 1..len]);
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![20, 12, 45, 19, 91, 55];
        quick_sort(&mut v);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
    }
    #[test]
    fn test_large_quick_sort() {
        use rand::Rng;
        let test_data: Vec<usize> = (0..100).map(|_| {
            rand::thread_rng().gen_range(1, std::usize::MAX)
        }).collect();
        let mut v = test_data.clone();
        quick_sort(&mut v);
        println!("{:?}", v);
    }

}
