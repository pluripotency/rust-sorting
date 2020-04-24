# Insertion Sort

```rust
{{#include ../src/insertion.rs:0:15}}

let mut v = vec![20, 12, 45, 19, 91, 55];
#println!("before: {:?}", v);
insertion_sort(&mut v, &|a, b| a < b);
#println!("sorted: {:?}", v);
#assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
```
