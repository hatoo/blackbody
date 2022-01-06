# blackbody

Blackbody simply provides functions that convert temperature into color.
Codes are ported from [pbrt-v3](https://github.com/mmp/pbrt-v3).

```rust
use blackbody::{temperature_to_rgb, temperature_to_xyz};

fn main() {
    // Convert temperatue (Kelvin) into RGB color whose components are [0.0, 1.0].
    dbg!(temperature_to_rgb(3000.0));
    // Convert temperatue (Kelvin) into XYZ color whose components are [0.0, 1.0].
    dbg!(temperature_to_xyz(3000.0));
}
```

```text
[examples\example.rs:4] temperature_to_rgb(3000.0) = [
    0.73506474,
    0.3504873,
    0.112983376,
]
[examples\example.rs:5] temperature_to_xyz(3000.0) = [
    0.44889182,
    0.4151354,
    0.16
]
```