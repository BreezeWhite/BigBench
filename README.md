# Rust Big Number Benchmark

This project compares different Rust big number crates in views of performance and precision.

The scenario leveraged for benchmarking is to calculate Pi to a thousand digits as close as possible. Algorithm applied to calculate Pi is [Bailey–Borwein–Plouffe formula (BBP)](https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula).

## Big Number Crates

Here compares the following Rust crates that are designed to deal with big numbers:

| Crate | Precision | Precision Type | Note |
| --- | --- | --- | --- |
| [bigdecimal](https://github.com/akubera/bigdecimal-rs) | arbitrary | decimal | Pure Rust. Need to set precision at compile time (see [ref](https://github.com/akubera/bigdecimal-rs?tab=readme-ov-file#compile-time-configuration)) |
| [rug](https://gitlab.com/tspiteri/rug) | arbitrary | binary | Depends on external GNU library |
| [rust-decimal](https://github.com/paupino/rust-decimal) | limited | binary | Pure Rust |
| [dashu](https://github.com/cmpute/dashu) | arbitrary | decimal | Pure Rust. |
| [num-bigfloat](https://github.com/stencillogic/num-bigfloat) | limited | binary | Pure Rust. Tricky to use (see [appendix](#careful-about-using-num_bigfloat)). |
| [astro-float](https://github.com/stencillogic/astro-float) | arbitrary | binary | Pure Rust. Extra efforts to use (see [appendix](#lack-supports-of-direct-arithmetic-manipulation)) |
| [fastnum](https://github.com/neogenie/fastnum) | limited | binary | Pure Rust |
| [decimal-rs](https://github.com/yashan-technologies/decimal-rs) | limited | binary | Pure Rust |
| [primitive_fixed_point_decimal](https://github.com/WuBingzheng/primitive_fixed_point_decimal) | limited | decimal | Pure Rust. Contributed by [WuBingzheng](https://github.com/WuBingzheng) in [PR #1](https://github.com/BreezeWhite/BigBench/pull/1) |
| [malachite](https://github.com/mhogrefe/malachite) | arbitrary | binary | Pure Rust. |
| [fixed-num](https://github.com/wdanilo/fixed-num) | limited | decimal | Pure Rust. High-precision, high-performance fixed-point decimal type. |


A raw Rust f64 version and also two Python versions using `decimal` are implemented to compare with these crates. 

## Experiment Setup

- All significant figures are set to 1,000 digits in decimal (if possible). For those precision refers to binary format, additional precision number converstion is calculated (multiplied with a constant log10 value).
- Iteration of Pi calculation is set to 1,000.
- There are two implementation types of Python `decimal` package, which are C and pure Python. Both results are provided (notice that the normal importing will use the C version).
- [hyperfine](https://github.com/sharkdp/hyperfine) is used for running the experiments.

## Correctness

Since not all approach supports arbitrary float number precision, an additional metric is listed to show how precise the Pi can be represented in terms of digits.

## Environment

- OS: Ubuntu 20.04
- CPU: Intel(R) Core(TM) i7-10700 CPU @ 2.90GHz
- RAM: 16GB
- Rust Version: 1.86.0
- Cargo Version: 1.86.0
- Python Version: 3.13.4

## Results

The results are as the following table:

| Approach | Runtime [ms] | Relative | Correctness (to nth digit)|
|:---|---:|---:|---:|
| raw Rust f64 | 0.786 ± 0.115 | 1.00 | 16 |
| `rust-decimal` | 8.307 ± 0.705 | 10.57 ± 1.79 | 28 |
| `fixed-num` | 12.878 ± 1.116 | 16.38 ± 2.79 | 19 |
| `bigdecimal` | 2286.325 ± 27.453 | 2908.21 ± 427.84 | 1000 |
| `rug` | 12.184 ± 1.044 | 15.50 ± 2.63 | 1000 |
| `dashu` | 28.392 ± 1.952 | 36.11 ± 5.85 | 1000 |
| `num-bigfloat` | 4.934 ± 0.493 | 6.28 ± 1.11 | 39 |
| `astro-float` | 34.588 ± 1.926 | 44.00 ± 6.90 | 1000 |
| `fastnum` | 585.688 ± 13.158 | 745.00 ± 110.51 | 307 |
| `decimal-rs` | 26.162 ± 1.362 | 33.28 ± 5.18 | 37 |
| `primitive_fixed_point_decimal` | 14.734 ± 0.841 | 18.74 ± 2.95 | 35 |
| `malachite` | 75.103 ± 2.553 | 95.53 ± 14.38 | 1000 |
| Python `decimal` (libmpdec) | 64.440 ± 83.759 | 81.97 ± 107.22 | 1000 |
| Python `decimal` (_pydecimal) | 504.321 ± 145.669 | 641.50 ± 207.80 | 1000 |

![figure](./results.png)

- The fastest one is the raw Rust f64 approach, but it has also the worst results in correctness, which is expected.
- The second and the third are `rust-decimal` and `num_bigfloat`, for which both use fix-length representation and thus the correctness are also limited.
- `fixed-num` is also extremely fast (ranking near the top of the list after `rust-decimal`), but its correctness is limited to 19 digits because `Dec19x19` has a fixed precision of exactly 19 fractional decimal digits.
- `decimal-rs` is much slower than the above two crates, which they all use fix-length of number representation.
- `fastnum` also uses a fixed-length representation, but it is significantly slower than all other approaches. This may be due to implicit cloning of shared objects, although it does offer higher precision than other fixed-length solutions.
- `primitive_fixed_point_decimal` puses a different underlying representation, and its performance is roughly comparable to that of `decimal-rs`. See more details about the differences [here](https://github.com/WuBingzheng/primitive_fixed_point_decimal/blob/master/COMPARISON.md).
- `rug` is the fastest among approaches that support arbitrary precision, though it depends on external GNU libraries. Note that the precision setting in the code refers to binary digits, so you need to manually convert the precision value to achieve the desired number of decimal digits.
- Both `dashu` and `astro-float` have roughly the same level of speed, for `dashu` is slightly faster over `astro-float`. But for the user experience, `astro-float` is much more lengthy to write compared to `dashu`. `astro-float` leverages a functional way for arithmetic operations. On the other hand, `dashu` could do it in a normal way of using arithmetic signs.
- Somewhat surprisingly, Python’s `decimal` outperforms the other three Rust crates, largely due to its underlying C implementation. When using the pure Python version, however, performance drops dramatically.
- Unexpectedly, `bigdecimal` has the worst performance, even compared to the pure Python `decimal` implementation. Its precision setting also differs from the other approaches, as it is determined at compile time. Although the documentation mentions that you can specify precision in code, doing so requires additional effort.

## Conclusion

Overall, if you are doing extremely precise floating-point calculation, `rug` is suggested. **DO** bare in mind to tune the digit of precision to your own needs.

The second recommended choice goes to `dashu`, for its effciency, precision, ease of use, and pure Rust support.


## Appendix

### Careful about using `num_bigfloat`

While calculating the Pi, using `num::bigfloat::from_u8(1)` to initialize inner varialbes might lead to incorrect results. To derive the correct results, it is required to use `num_bigfloat::ONE` and multiply it to get other integer numbers for the calculation. This behavior is weired. 

### Lack Supports of Direct Arithmetic Manipulation

When using `astro-float`, to do normal arithmetic calculation like add and multiply, you need to write it in a functional way with addtional arguments like following: 
```rust
use astro_float::{BigFloat, RoundingMode};

let prec = 1000;
let rm = RoundingMode::None;
let a = BigFloat::from_u8(2, prec);
let b = BigFloat::from_u8(3, prec);
let c = a.add(b, prec, rm);

// Following code does not compile
let c = a + b;
```

Or you have to use the `expr!` macro to accomplish it
```rust
use astro_float::ctx::Context;
use astro_float::{expr, Consts};

let mut ctx = Context::new(
    prec,
    rm,
    Consts::new().expect("constants cache initialized"),
    -10000,
    10000,
);

let c = expr!(a + b, ctx);
```
