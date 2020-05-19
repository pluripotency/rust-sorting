use core::mem;
use core::ptr;

////////////////////////////////////////////////////////////////////////////////
// Sorting
////////////////////////////////////////////////////////////////////////////////

/// Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.
///
/// This is the integral subroutine of insertion sort.
pub fn insert_head<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if v.len() >= 2 && is_less(&v[1], &v[0]) {
        unsafe {
            // There are three ways to implement insertion here:
            //
            // 1. Swap adjacent elements until the first one gets to its final destination.
            //    However, this way we copy data around more than is necessary. If elements are big
            //    structures (costly to copy), this method will be slow.
            //
            // 2. Iterate until the right place for the first element is found. Then shift the
            //    elements succeeding it to make room for it and finally place it into the
            //    remaining hole. This is a good method.
            //
            // 3. Copy the first element into a temporary variable. Iterate until the right place
            //    for it is found. As we go along, copy every traversed element into the slot
            //    preceding it. Finally, copy data from the temporary variable into the remaining
            //    hole. This method is very good. Benchmarks demonstrated slightly better
            //    performance than with the 2nd method.
            //
            // All methods were benchmarked, and the 3rd showed best results. So we chose that one.
            let mut tmp = mem::ManuallyDrop::new(ptr::read(&v[0]));

            // Intermediate state of the insertion process is always tracked by `hole`, which
            // serves two purposes:
            // 1. Protects integrity of `v` from panics in `is_less`.
            // 2. Fills the remaining hole in `v` in the end.
            //
            // Panic safety:
            //
            // If `is_less` panics at any point during the process, `hole` will get dropped and
            // fill the hole in `v` with `tmp`, thus ensuring that `v` still holds every object it
            // initially held exactly once.
            let mut hole = InsertionHole { src: &mut *tmp, dest: &mut v[1] };
            ptr::copy_nonoverlapping(&v[1], &mut v[0], 1);

            for i in 2..v.len() {
                if !is_less(&v[i], &*tmp) {
                    break;
                }
                ptr::copy_nonoverlapping(&v[i], &mut v[i - 1], 1);
                hole.dest = &mut v[i];
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }

    // When dropped, copies from `src` into `dest`.
    struct InsertionHole<T> {
        src: *mut T,
        dest: *mut T,
    }

    impl<T> Drop for InsertionHole<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::copy_nonoverlapping(self.src, self.dest, 1);
            }
        }
    }
}

pub fn insertion_sort<T, F>(v: &mut [T], mut is_less: F)
where
    F: FnMut(&T, &T)-> bool,
{
    let len = v.len();
    if len >= 2 {
        for i in (0..len - 1).rev() {
            insert_head(&mut v[i..], &mut is_less);
        }
        return
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        let test_data = vec![81.0, 20.0, 13.0, 32.0, 62.0, 54.0, 8.0, 95.0, 73.0, 19.0, 90.0, 23.0, 48.0, 6.0, 3.0];
        let test_answer = vec![3.0, 6.0, 8.0, 13.0, 19.0, 20.0, 23.0, 32.0, 48.0, 54.0, 62.0, 73.0, 81.0, 90.0, 95.0];
        let mut v = test_data.clone();
        insertion_sort(&mut v, |a, b| a.lt(b));
        assert_eq!(test_answer, v);
    }
}
