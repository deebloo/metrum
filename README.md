# metrum

Easily handle measurement values in rust.

The goal is to work with measurements without needing to think about about units until you absolutely have to.

## Length

| Unit | Constructor | Conversion |
| ---- | ----------- | ---------- |
|      |             |            |


### Units

| Unit        | Constructor           | Conversion    |
| ----------- | --------------------- | ------------- |
| Celcius     | `Temp::from_c(value)` | `temp.as_c()` |
| Farenheight | `Temp::from_f(value)` | `temp.as_f()` |
| Kelvin      | `Temp::from_k(value)` | `temp.as_k()` |

## Speed

| Unit  | Constructor               | Conversion        |
| ----- | ------------------------- | ----------------- |
| MPS   | `Temp::from_mps(value)`   | `temp.as_mps()`   |
| KPH   | `Temp::from_kph(value)`   | `temp.as_kph()`   |
| MPH   | `Temp::from_mph(value)`   | `temp.as_mph()`   |
| Knots | `Temp::from_knots(value)` | `temp.as_knots()` |

## Temp

| Unit        | Constructor           | Conversion    |
| ----------- | --------------------- | ------------- |
| Celcius     | `Temp::from_c(value)` | `temp.as_c()` |
| Farenheight | `Temp::from_f(value)` | `temp.as_f()` |
| Kelvin      | `Temp::from_k(value)` | `temp.as_k()` |


```rust
use metrum::Temp;
use metrum::temp::TempDelta;

let from_f = Temp::from_f(32.);
let from_c = Temp::from_c(0.);

assert_eq!(from_f, from_c);
assert_eq!(from_f - from_c, TempDelta { value: 0. });
```

## Weight

| Unit | Constructor | Conversion |
| ---- | ----------- | ---------- |
|      |             |            |


## Serde
Values can be safely serialized and deserialized using serde when the `serde` feature is enabled.

```toml
[dependencies]
metrum = { version = "0.5.0", features = ["serde"] }
```
