pub fn bubble_sort<T, U>(array: &mut Vec<T>, comparator: U)
    where
        T: PartialOrd,
        U: Fn(&T, &T) -> bool,
{
    let length = array.len();
    for i in 0..(length - 1) {
        for j in (i + 1)..length {
            if comparator(&array[j], &array[i]) {
                array.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        fn order_ascend<T: PartialOrd>(a: &T, b: &T) -> bool {
            a < b
        }
        let mut v = vec![20, 12, 45, 19, 91, 55];
        bubble_sort(&mut v, order_ascend);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
    }

    #[test]
    fn test_bubble_sort_using_closure() {
        let mut v = vec![20, 12, 45, 19, 91, 55];
        bubble_sort(&mut v, |a, b| a < b);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
    }

    #[test]
    // to display,
    // cargo test -- --nocapture
    fn test_bubble_sort_using_output_comparator() {
        use std::fmt::{Debug, Display};
        fn order_ascend<T: PartialOrd + Display>(a: &T, b: &T) -> bool {
            println!("test: {} < {}", a, b);
            a < b
        }
        fn output<T: Debug>(array: Vec<T>) {
            let length = array.len();
            println!(
                "length: {} compared {} times(n*(n-1)/2)",
                length,
                length * (length - 1) / 2
            );
            println!("{:?}", array);
        }
        let mut v = vec![20, 12, 45, 19, 91, 55];
        bubble_sort(&mut v, order_ascend);
        assert_eq!(vec![12, 19, 20, 45, 55, 91], v);
        output(v);
    }
}
