use std::ops::BitXor;

#[allow(dead_code)]
pub fn swap_by_xor<T>(array: &mut Vec<T>, from: usize, to: usize)
where
    T: Copy + BitXor<Output = T>,
{
    // XOR trick can be used only for numbers
    array[from] = array[from] ^ array[to];
    array[to] = array[from] ^ array[to];
    array[from] = array[from] ^ array[to];
}

#[allow(dead_code)]
pub fn swap<T: Copy>(array: &mut Vec<T>, a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_by_xor() {
        let mut a_b = vec![0, 1];
        swap_by_xor(&mut a_b, 0, 1);
        assert_eq!(vec![1, 0], a_b)
    }

    #[test]
    fn test_swap() {
        let mut a_b = vec![0, 1];
        swap(&mut a_b, 0, 1);
        assert_eq!(vec![1, 0], a_b)
    }
}
