# Quick Sort

```rust
{{#include ../src/quick.rs:1:26}}

let mut v = vec![20, 12, 45, 19, 91, 55];
#println!("before: {:?}", v);
quick_sort(&mut v);
#println!("sorted: {:?}", v);
#assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
```
