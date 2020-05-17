use std::mem;
use std::cmp;
use std::ptr;

pub fn sort<T: PartialOrd + Clone>(source: &mut [T]) {
    const INSERT_THRESHOLD: usize = 32;

    fn partial_insert_sort<TInner: PartialOrd>(source: &mut [TInner], size: usize) {
        let ptr = source.as_mut_ptr();
        let mut left = 0;
        while left < source.len() {
            let right = cmp::min(left + size, source.len());
            for i in left..right {
                let mut j = i;
                unsafe {
                    let mut t: TInner = mem::uninitialized();
                    ptr::copy_nonoverlapping(ptr.offset(j as isize), &mut t, 1);
                    while left < j && *(ptr.offset((j - 1) as isize)) > t {
                        ptr::copy_nonoverlapping(
                            ptr.offset((j - 1) as isize),
                            ptr.offset(j as isize),
                            1,
                        );
                        j -= 1;
                    }
                    ptr::copy_nonoverlapping(&t, ptr.offset(j as isize), 1);
                    mem::forget(t);
                }
            }
            left = right;
        }
    }

    fn sort_inner<TInner: PartialOrd + Clone>(
        work_source: &[TInner],
        work: &mut [TInner],
        size: usize,
    ) {
        let band = size << 1;
        let mut write_pos = 0;
        let b = work_source.len() - size;

        let mut j = 0;
        while j < b {
            let mid = j + size;
            let end = cmp::min(j + band, work_source.len());
            let mut left_pos = j;
            let mut right_pos = mid;

            loop {
                if work_source[left_pos] <= work_source[right_pos] {
                    work[write_pos] = work_source[left_pos].clone();
                    left_pos += 1;
                    write_pos += 1;
                    if left_pos >= mid {
                        while right_pos < end {
                            work[write_pos] = work_source[right_pos].clone();
                            write_pos += 1;
                            right_pos += 1;
                        }
                        break;
                    }
                } else {
                    work[write_pos] = work_source[right_pos].clone();
                    right_pos += 1;
                    write_pos += 1;
                    if right_pos >= end {
                        while left_pos < mid {
                            work[write_pos] = work_source[left_pos].clone();
                            write_pos += 1;
                            left_pos += 1;
                        }
                        break;
                    }
                }
            }
            j += band;
        }

        if j < work_source.len() {
            work[j..work_source.len()].clone_from_slice(&work_source[j..work_source.len()]);
        }
    }

    // 初めにINSERT_THRESHOLDサイズで挿入ソートを行う
    partial_insert_sort(source, INSERT_THRESHOLD);

    if source.len() <= INSERT_THRESHOLD {
        return;
    }

    let mut work = source.to_vec(); // 配列の初期化
    let mut i = INSERT_THRESHOLD;
    loop {
        if i < source.len() {
            sort_inner(source, &mut work, i);
            i <<= 1;
        } else {
            return;
        }
        if i < source.len() {
            sort_inner(&work, source, i);
            i <<= 1;
        } else {
            source.clone_from_slice(&work);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let test_data = vec![81.0, 20.0, 13.0, 32.0, 62.0, 54.0, 8.0, 95.0, 73.0, 19.0, 90.0, 23.0, 48.0, 6.0, 3.0];
        let test_answer = vec![3.0, 6.0, 8.0, 13.0, 19.0, 20.0, 23.0, 32.0, 48.0, 54.0, 62.0, 73.0, 81.0, 90.0, 95.0];
        let mut v = test_data.clone();
        sort(&mut v);
        assert_eq!(test_answer, v);
    }
}
