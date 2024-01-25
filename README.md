# Fuzed iterator

[![fuzed-iterator at crates.io](https://img.shields.io/crates/v/fuzed-iterator.svg)](https://crates.io/crates/fuzed-iterator)
[![fuzed-iterator at docs.rs](https://docs.rs/fuzed-iterator/badge.svg)](https://docs.rs/fuzed-iterator)
[![Build status](https://github.com/magicant/fuzed-iterator-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/magicant/fuzed-iterator-rs/actions/workflows/rust.yml)
[![Changelog](https://img.shields.io/badge/Changelog-gray)](CHANGELOG.md)

`fuzed-iterator` is a Rust library crate that provides a simple iterator wrapper
that causes a panic if the iterator is used after it has once returned `None`.

Many iterators in [`std`] implement the [`FusedIterator`] trait, which
guarantees that the iterator will never return `Some` after returning `None`.
This may make programmers incorrectly assume that any iterator works like a
`FusedIterator`, leading to bugs when a non-fused iterator is used.

The `Fuze` wrapper provided by this crate can be used to wrap any iterator,
causing a panic if the iterator is used after returning `None` for the first
time. This can be used to catch bugs where a non-fused iterator is used in a
context where a fused iterator is expected. In your unit tests (or integration
tests), pass iterators through `Fuze` to your functions that expect an iterator
(not necessarily fused), and the tests will panic if the function calls
`Iterator::next` excessively.

## Example

```rust should_panic
use fuzed_iterator::IteratorExt as _;
let mut iter = (0..3).fuze();
assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);
iter.next(); // Another `next` call would panic!
```

```rust should_panic
/// Collects items from an iterator into a vector, but drops the first item.
fn drop_first_and_collect<I: IntoIterator<Item = i32>>(i: I) -> Vec<i32> {
    // This implementation is wrong because `next` may be called again even after it
    // returned `None`.
    let mut i = i.into_iter();
    _ = i.next();
    i.collect()
}

// Because of the wrong implementation, this test case would fail with a panic.
# /*
#[test]
# */
fn test_drop_first_and_collect_with_empty_array() {
    use fuzed_iterator::IteratorExt as _;
    let result = drop_first_and_collect([].into_iter().fuze());
    assert_eq!(result, []);
}
# test_drop_first_and_collect_with_empty_array();
```

```rust
/// Collects items from an iterator into a vector, but drops the first item.
fn drop_first_and_collect<I: IntoIterator<Item = i32>>(i: I) -> Vec<i32> {
    // This is the correct implementation.
    let mut i = i.into_iter();
    if i.next().is_none() {
        return vec![];
    }
    i.collect()
}

// Test passed!
# /*
#[test]
# */
fn test_drop_first_and_collect_with_empty_array() {
    use fuzed_iterator::IteratorExt as _;
    let result = drop_first_and_collect([].into_iter().fuze());
    assert_eq!(result, []);
}
# test_drop_first_and_collect_with_empty_array();
```

## License

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-Apache), at your option

[`std`]: https://doc.rust-lang.org/std/index.html
[`FusedIterator`]: https://doc.rust-lang.org/std/iter/trait.FusedIterator.html
