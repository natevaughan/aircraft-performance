# Aircraft Performance Calculator

Replace aircraft performance calculation charts with an automatic calculation!

Currently only supports Piper Arrow III

## Requirements

 - Rust
 - Cargo

## Running

The app requires 4 ordered arguments:
 - Outside air temp (celsius)
 - Field elevation (ft)
 - Aircraft takeoff weight (lbs)
 - Headwind (positive kts) or tailwind (negative knots)

Example:
```
cargo run 10 740 2650 6
```

## Building for release

```
cargo build -r
```

Resulting executable will be available in target/release/aircraft-performance