# Calculator

A simple calculator application for CachyOS (KDE Plasma, Qt6) built with Rust and the [iced](https://github.com/iced-rs/iced) GUI framework.

## Features

- Basic arithmetic: addition, subtraction, multiplication, division
- Chained operations (e.g., `2 + 3 × 4`)
- Percentage calculations
- Negation
- Clear / Clear Entry
- Division-by-zero protection
- Overflow detection

## Requirements

- Rust toolchain (stable channel)
- KDE Plasma with Qt6 support (for the GUI)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Testing

Run all tests (unit, integration, and doc tests):

```bash
cargo test
```

### Test Summary

| Test Suite | Count | Description |
|------------|-------|-------------|
| Unit tests | 31 | Calculator engine logic (`src/calculator.rs`) |
| Integration tests | 42 | Public API surface (`tests/calculator_engine.rs`) |
| Doc tests | 1 | Public API examples in doc comments |

### Test Categories

**Basic Arithmetic**
- Addition, subtraction, multiplication, division of two numbers
- Chained operations with three or more numbers
- Mixed operations (e.g., `2 + 3 × 4`)

**Division by Zero**
- Returns `CalculatorError::DivisionByZero`
- Does not crash

**Decimal Numbers**
- Decimal point entry
- Decimal arithmetic operations

**Edge Cases**
- Zero result
- Negative result
- Large number multiplication
- Small decimal results
- Equals without pending operation

**Error Handling**
- `CalculatorError` implements `std::error::Error`
- Clone, Eq, PartialEq derived
- Display messages for all error variants

## Project Structure

```
Cargo.toml          # Package manifest
src/
  main.rs           # Entry point + module declarations
  calculator.rs     # Calculator engine + unit tests
  app.rs            # iced GUI application
tests/
  calculator_engine.rs  # Integration tests
```

## License

MIT