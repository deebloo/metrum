# metrum

Easily handle measurement values in rust.

The goal is to work with measurements without needing to think about about units at all.

For example: A `Temp` is itself a value. The units are just for display back to the user.

You can compare and combine temps safely without manually converting to and from different units.

For example you can subtract 10degC from 86degF. The crate will handle the conversions internally and give you a result in the initial unit.

```rust
use metrum:Temp;

let value = Temp::F(86.) - Temp::C(10.);

println("{}", value.as_k()); // 275.372
```

This also means you can safely compare temperatures in two different units

```rust
use metrum:Temp;

let value = Temp::F(86.) == Temp::C(30.); // true
```

Temps can be safely serialized and deserialized using serde when the `serde` feature is enabled.

```toml
[dependencies]
metrum = { version = "0.2.0", features = ["serde"] }
```
