# Quick Sort

```rust
{{#include ../src/swap.rs:13:17}}

{{#include ../src/quick.rs:3:53}}

let mut v = vec![20, 12, 45, 19, 91, 55];
#println!("before: {:?}", v);
let length = v.len();
quick_sort(&mut v, 0, length - 1);
#println!("sorted: {:?}", v);
#assert_eq!(vec![12, 19, 20, 45, 55, 91], v)
```
