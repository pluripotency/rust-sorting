# Merge Sort

```rust
{{#include ../src/merge_recurse_rosetta.rs:0:38}}

let mut v = vec![81, 20, 13, 32, 62, 54, 8, 95, 73, 19, 90, 23, 48, 6, 3];
#println!("before: {:?}", v);
merge_sort(&mut v);
#println!("sorted: {:?}", v);
#assert_eq!(vec![3, 6, 8, 13, 19, 20, 23, 32, 48, 54, 62, 73, 81, 90, 95], v);
```
