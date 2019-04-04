<a href="https://github.com/open-flash/open-flash">
    <img src="https://raw.githubusercontent.com/open-flash/open-flash/master/logo.png"
    alt="Open Flash logo" title="Open Flash" align="right" width="64" height="64" />
</a>

# SWF Fixed

[![crates.io](https://img.shields.io/crates/v/swf-fixed.svg?maxAge=86400)](https://crates.io/crates/swf-fixed)
[![GitHub repository](https://img.shields.io/badge/Github-open--flash%2Frust--swf--fixed-blue.svg?maxAge=86400)](https://github.com/open-flash/rust-swf-fixed)
[![Build status](https://img.shields.io/travis/open-flash/rust-swf-fixed/master.svg?maxAge=86400)](https://travis-ci.org/open-flash/rust-swf-fixed)

SWF fixed-point numbers for Rust.

A fixed point number represents a decimal values using evenly-distributed bit
patterns (as opposed to floating point numbers where the density increases with
the proximity to zero).

A fixed point number can be simply thought as an integer divided by a constant value.
It is described by its integer part and fractional part:
its mathematical value is `integer_part + fractional_part / 2^fractional_bits`.

For example, the type `Ufixed8p8` is an unsigned fixed point number with an
8 bit integer part and 8 bit fractional part. It can represent the 2^16 values
corresponding to `u16 / 256`, the gap between each value (epsilon) is `1 / 256`.

This crate defines the fixed points numbers used by SWF files:

| Name        | Integer part | Fractional part | Min value | Max value     | Epsilon  |
|-------------|--------------|-----------------|-----------|---------------|----------|
| Sfixed8P8   | `i8`         | `u8`            | -128      | 128 - 1/256   | 1 / 256  |
| Ufixed8p8   | `u8`         | `u8`            | 0         | 256 - 1/256   | 1 / 256  |
| Sfixed16p16 | `i16`        | `u16`           | -2^15     | 2^15 - 1/2^16 | 1 / 2^16 |
| Ufixed16p16 | `u16`        | `u16`           | 0         | 2^16 - 1/2^16 | 1 / 2^16 |

## Usage

```rust
use swf_fixed::Sfixed8P8;

fn main() {
  let a = Sfixed8P8::from_epsilons(256);
  let b = Sfixed8P8::from_value(1f32);
  assert_eq!(a, b);
  let sum: Sfixed8P8 = (a + b);
  let sum_value: f32 = sum.into();
  assert_eq!(sum_value, 2.0f32);
}
```

## Contributing

This library is a standard Cargo project. You can test your changes with
`cargo test`.

Prefer non-`master` branches when sending a PR so your changes can be rebased if
needed. All the commits must be made on top of `master` (fast-forward merge).
CI must pass for changes to be accepted.
