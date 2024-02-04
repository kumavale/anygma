# anygma 🦝

[![Actions Status](https://github.com/kumavale/anygma/workflows/CI/badge.svg)](https://github.com/kumavale/anygma/actions)
[![Crates.io](https://img.shields.io/crates/v/anygma.svg)](https://crates.io/crates/anygma)
[![Documentation](https://docs.rs/anygma/badge.svg)](https://docs.rs/anygma)

`anygma` makes it easy to define arrays containing different types.

## Examples

```rust
use anygma::ary_anyref;

let a = ary_anyref![0, 'a', "str"];
assert_eq!(a[0].downcast_ref::<i32>(), Some(&0));
assert_eq!(a[1].downcast_ref::<char>(), Some(&'a'));
assert_eq!(a[2].downcast_ref::<&str>(), Some(&"str"));
```

## Contributing

This project welcomes your PR and issues. For example, fixing bugs, adding features, refactoring, etc.
