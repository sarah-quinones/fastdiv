# fastdiv
This crate performs fast division by a runtime constant divisor,
by precomputing a division factor that can then be used repeatedly.
We provide fast division for both u32 and u64.

# Example
```rust
use fastdiv::FastDiv;

let d: u32 = 3;
let m = d.precompute_div();

let n1 = 4;
let n2 = 9;

assert_eq!(n1 / d, n1.fast_div(m));
assert_eq!(n2 / d, n2.fast_div(m));

assert_eq!(n1 % d, n1.fast_mod(m, d));
assert_eq!(n2 % d, n2.fast_mod(m, d));

assert_eq!(n1 % d == 0, n1.is_multiple_of(m));
assert_eq!(n2 % d == 0, n2.is_multiple_of(m));
```

# Benchmarks
Benchmarks can be executed with `cargo bench`.
The results on my `i5-11400 @ 2.60GHz` are:
```
fast div u32            time:   [484.54 ps 484.74 ps 484.93 ps]
slow div u32            time:   [1.3677 ns 1.3685 ns 1.3697 ns]
const mod u32           time:   [485.90 ps 487.29 ps 489.74 ps]

fast mod u32            time:   [640.64 ps 641.03 ps 641.47 ps]
slow mod u32            time:   [1.3670 ns 1.3675 ns 1.3681 ns]
const mod u32           time:   [536.29 ps 536.99 ps 537.92 ps]


fast div u64            time:   [683.69 ps 683.97 ps 684.31 ps]
slow div u64            time:   [2.2783 ns 2.2792 ns 2.2803 ns]
const div u64           time:   [566.71 ps 569.92 ps 573.22 ps]

fast mod u64            time:   [1.0259 ns 1.0264 ns 1.0270 ns]
slow mod u64            time:   [2.2786 ns 2.2792 ns 2.2799 ns]
const mod u64           time:   [598.19 ps 598.36 ps 598.55 ps]
```
