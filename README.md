# Fuzed iterator

<!-- TODO badges -->

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
use fuzed_iterator::IteratorExt;
let mut iter = (0..3).fuze();
assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);
iter.next(); // Another `next` call would panic!
```

## License

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-Apache), at your option

[`std`]: https://doc.rust-lang.org/std/index.html
[`FusedIterator`]: https://doc.rust-lang.org/std/iter/trait.FusedIterator.html
