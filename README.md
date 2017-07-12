# unsafe_unwrap

A Rust library that enables unchecked unwrapping on `Option` and `Result` types.

## Usage

The `unsafe_unwrap()` method can be used anywhere `unwrap()` is used. It behaves
similar to `unwrap()` in unoptimized builds and will remove checks in optimized
builds.

```rust
extern crate unsafe_unwrap;
use unsafe_unwrap::UnsafeUnwrap;

let x = Some(42);
let y = unsafe { x.unsafe_unwrap() };
```

## Benchmark

| `bench_normal_unwrap_1000` | `bench_unsafe_unwrap_1000` |
| -------------------------- | -------------------------- |
| 929 ns/iter (+/- 176)      | 302 ns/iter (+/- 28)       |

## License

This project is released under either:

- [MIT License][license-mit]

- [Apache License (Version 2.0)][license-apache]

at your choosing.

[license-mit]: https://github.com/nvzqz/unsafe-unwrap-rs/blob/master/LICENSE-MIT
[license-apache]: https://github.com/nvzqz/unsafe-unwrap-rs/blob/master/LICENSE-APACHE
