# iter-n

[![Crates.io](https://img.shields.io/crates/v/iter-n.svg)](https://crates.io/crates/iter-n)
[![Docs.rs](https://docs.rs/iter-n/badge.svg)](https://docs.rs/iter-n/)
[![Actions Status](https://github.com/frozenlib/iter-n/workflows/CI/badge.svg)](https://github.com/frozenlib/iter-n/actions)

A utility for functions returning impl Iterator to return one of several distinct types.

## Motivation

In functions that return `impl Iterator`, it is necessary to return an iterator of a specific type. Therefore, it is not possible to return an iterator of a different type, as shown below.

```rust
fn f(x: i32) -> impl Iterator<Item = i32> {
    if x % 2 == 0 {
        [0, 1].iter().map(|y| y + 1)
    } else {
        [0, 1].iter().map(|y| y + 2) // ERROR: `if` and `else` have incompatible types
    }
}
```

By using `iter_n`, you can return an iterator of a different type from a function.

## Example

`use iter_n::iter2::*` must be placed in function scope, not in module scope.

Since `iter_n::iter2`, `iter_n::iter3`, etc. define methods of the same name, if multiple `use iter_n::iter{N}::*;` are placed in the module scope, there will be a conflict with the methods.

```rust
fn f(x: i32) -> impl Iterator<Item = i32> {
    use iter_n::iter2::*;
    if x % 2 == 0 {
        [0, 1].iter().map(|y| y + 1).into_iter0()
    } else {
        [0, 1].iter().map(|y| y + 2).into_iter1()
    }
}
```

```rust
fn g(x: i32) -> impl Iterator<Item = i32> {
    use iter_n::iter3::*;
    if x % 3 == 0 {
        [0, 1].iter().map(|y| y + 1).into_iter0()
    } else if x % 3 == 0 {
        [0, 1].iter().map(|y| y + 2).into_iter1()
    } else {
        [0, 1].iter().map(|y| y + 3).into_iter2()
    }
}
```

## License

This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
