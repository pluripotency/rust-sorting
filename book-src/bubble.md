# Bubble Sort

```rust
use std::ops::BitXor;

pub fn swap<T>(array: &mut Vec<T>, from: usize, to: usize)
{{#include ../src/swap.rs:4:11}}

{{#include ../src/bubble.rs:4:17}}

let mut v = vec![20, 12, 45, 19, 91, 55];
#println!("before: {:?}", v);
bubble_sort(&mut v, |a, b| a < b);
#println!("sorted: {:?}", v);
#assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
```

