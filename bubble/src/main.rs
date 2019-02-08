use std::ops::BitXor;
use std::fmt::{Debug, Display};

fn swap<T>(array: &mut Vec<T>, from: usize, to: usize)
where T: Copy + BitXor<Output = T>
{
    // XOR trick can be used only for numbers
    array[from] = array[from] ^ array[to];
    array[to] = array[from] ^ array[to];
    array[from] = array[from] ^ array[to];
}

fn bubble_sort<T, U>(array: &mut Vec<T>, comparator: U)
where
    T: Display + Copy + BitXor<Output = T>,
    U: Fn(T, T)->bool
{
    let length = array.len();
    for i in 0..(length-1) {
        for j in (i+1)..length {
            println!("test: {}: {} > {}: {}", i, array[i], j, array[j]);
            if comparator(array[j], array[i]) {
                swap(array, i, j);
            }
        }
    }
}

fn output<T: Debug>(array: Vec<T>){
    let length = array.len();
    println!("length: {} compared {} times(n*(n-1)/2)", length, length * (length-1) / 2);
    println!("{:?}", array);
}

fn order_ascend<T: Ord>(a: T, b: T)-> bool { a < b }

fn main() {
    let use_closure = true;
    let mut v = vec![20, 12, 45, 19, 91, 55];
    if use_closure {
        bubble_sort(&mut v, |a,b| a<b);
    }else{
        bubble_sort(&mut v, order_ascend);
    }
    output(v);

}
