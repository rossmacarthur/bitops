# bitops

[![crates.io](https://img.shields.io/crates/v/bitops.svg?longCache=true&colorB=4d76ae)](https://crates.io/crates/bitops)
[![docs.rs](https://docs.rs/bitops/badge.svg)](https://docs.rs/bitops)

Miscellaneous bit operations for any Integer. See documentation [here](https://docs.rs/bitops).

## Getting started

Add to your project with

```bash
cargo add bitops
```

or directly editing your `Cargo.toml`

```toml
[dependencies]
bitops = "0.1.0"
```

## Example usage

```rust
use bitops::BitOps;

let x = 0b1010_1011_0000_1100; // 0xab0c
let flag = 0b1000;

assert!(flag.is_flag());
assert!(flag.is_bit_set(3));

assert!(x.is_flag_set(flag));
assert_eq!(x.bits_as_int(8, 4), 0xb);
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
