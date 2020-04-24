# Bubble Sort

```rust
{{#include ../src/bubble.rs:1:14}}

let mut v = vec![20, 12, 45, 19, 91, 55];
#println!("before: {:?}", v);
bubble_sort(&mut v, |a, b| a < b);
#println!("sorted: {:?}", v);
#assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
```

