# int_cmp
Rust Integer Types Comparison Library

## Example

```rust, no_run
use int_cmp::IntCmp;

let a = -25_i8;
let b = 64_u8;
assert!(a.cmp_lt(b));

let x = -45000_i32;
let y = 2560000_u64;
assert!(x.cmp_ne(y));
```

# TODOs
- Add support for usize

# Contributing
Any Pull Request is welcome, however small your contribution may be!
