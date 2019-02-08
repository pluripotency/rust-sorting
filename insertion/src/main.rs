fn insertion_sort<T: Ord + Copy, U: Fn(T, T)->bool>(array: &mut Vec<T>, comparator: U) {
    for i in 1..array.len() {
        let num_to_insert = array[i];
        let mut compare_index = i;
        while (compare_index > 0) && comparator(num_to_insert, array[compare_index-1]) {
            array[compare_index] = array[compare_index - 1];
            compare_index -= 1;
        }
        array[compare_index] = num_to_insert;
    }
}

fn main() {
    let mut v = vec![20, 12, 45, 19, 91, 55];
    insertion_sort(&mut v, |a,b| a < b);
    println!("{:?}", v);
}
