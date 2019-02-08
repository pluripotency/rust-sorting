fn insertion_sort(array: &mut Vec<usize>){
    let length = array.len();
    for i in 1..length {
        let num_to_insert = array[i];
        let mut compare_index = i.clone();
        while (compare_index > 0) && (array[compare_index-1] > num_to_insert) {
            array[compare_index] = array[compare_index - 1];
            compare_index -= 1;
        }
        array[compare_index] = num_to_insert;
    }
}

fn main() {
    let mut v = vec![20, 12, 45, 19, 91, 55];
    insertion_sort(&mut v);
    println!("{:?}", v);
}
