fn swap(array: &mut Vec<usize>, a: usize, b: usize){
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}
fn pivot(array: &mut Vec<usize>, i: usize, j: usize)-> usize {
    let x = array[i];
    let y = array[i + (j-i)/2];
    let z = array[j];
    if x < y {
        if y < z {
            y
        } else if z < x {
            x
        } else {
            z
        }
    }else{
        if z < y {
            y
        } else if x < z {
            x
        } else {
            z
        }
    }
}
fn partition(array: &mut Vec<usize>, i: usize, j: usize, p: usize)-> usize {
    let mut l = i.clone();
    let mut r = j.clone();
    loop {
        while array[l] < p { l += 1; }
        while p < array[r] { r -= 1; }
        if l > r { break; }
        swap(array, l, r);
        l += 1;
        r -= 1;
    }
    l
}

fn quick_sort(array: &mut Vec<usize>, i: usize, j: usize) {
    if i < j {
        let p = pivot(array, i, j);
        let k = partition(array, i, j, p);
        quick_sort(array, i, k-1);
        quick_sort(array, k, j);
    }
}

fn main() {
    let mut v = vec![20, 12, 45, 19, 91, 55];
    let length = v.len();
    quick_sort(&mut v, 0, length-1);
    println!("{:?}", v);
}
