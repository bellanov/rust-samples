# rust-samples

[![CI](https://github.com/bellanov/rust-samples/actions/workflows/ci.yml/badge.svg)](https://github.com/bellanov/rust-samples/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/bellanov/rust-samples/branch/main/graph/badge.svg)](https://codecov.io/gh/bellanov/rust-samples)

Sample apps and code written in the Rust programming language, demonstrating modern design patterns.

## Overview

This project showcases modern Rust design patterns and best practices, including:

- **Custom Error Handling**: Using `thiserror` for ergonomic error types
- **Trait-Based Abstractions**: Polymorphism through trait objects and generic bounds
- **Generic Programming**: Reusable, type-safe generic components
- **Zero-Cost Abstractions**: Leveraging Rust's compile-time features
- **Comprehensive Testing**: Unit tests with high coverage
- **CI/CD Pipeline**: Automated linting, building, testing, and coverage reporting

## Project Structure

```
rust-samples/
├── src/
│   ├── lib.rs          # Main library with design pattern examples
│   └── bin/
│       └── main.rs     # Example binary demonstrating library usage
├── Cargo.toml          # Project manifest and dependencies
└── .github/workflows/
    └── ci.yml          # GitHub Actions CI/CD pipeline
```

## Building

```bash
# Build the library
cargo build --lib

# Build the binary
cargo build --bin main

# Release build with optimizations
cargo build --release
```

## Running

```bash
# Run the example binary
cargo run --bin main

# Run with optimizations
cargo run --release --bin main
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_uppercase_processor

# Run tests including ignored
cargo test -- --include-ignored
```

## Code Coverage

Generate code coverage report using `cargo-tarpaulin`:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html --output-dir ./coverage

# Generate XML for CI
cargo tarpaulin --out Xml --output-dir ./coverage
```

## Linting and Formatting

```bash
# Check formatting
cargo fmt -- --check

# Auto-format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Check code
cargo check
```

## Design Patterns Demonstrated

### 1. Custom Error Types
Located in `src/lib.rs`, the `RustSamplesError` enum demonstrates:
- Using the `thiserror` crate for ergonomic error types
- Custom `Result` type alias for cleaner error handling
- Pattern matching for error handling

### 2. Trait-Based Abstraction
The `DataProcessor` trait shows:
- Interface design through traits
- Multiple implementations (`UppercaseProcessor`, `LowercaseProcessor`)
- Trait objects for dynamic dispatch (`Box<dyn DataProcessor>`)

### 3. Generic Programming
The `Container<T>` struct demonstrates:
- Generic type parameters with trait bounds
- Generic methods (`map`, `apply`)
- Type-safe abstractions

### 4. Error Handling Functions
- `parse_number()`: Demonstrates parse error handling with context
- `validate_string()`: Shows validation with custom error messages

## Dependencies

- **thiserror** - Ergonomic error handling
- **serde** (optional) - Serialization/deserialization
- **tokio** (dev) - Async runtime for testing
- **proptest** (dev) - Property-based testing

## CI/CD Pipeline

The GitHub Actions workflow (`.github/workflows/ci.yml`) includes:

1. **Check**: Verifies code compiles
2. **Format**: Checks code formatting with `rustfmt`
3. **Lint**: Runs `clippy` to catch common mistakes
4. **Test**: Executes all unit tests
5. **Coverage**: Generates code coverage and uploads to Codecov

All jobs run on the latest Ubuntu runner with Rust stable toolchain.

## Example Output

Running the binary produces:

```
Rust Samples - Modern Design Patterns Demo

=== Trait-Based Abstraction ===
UppercaseProcessor: 'Rust is Awesome' -> 'RUST IS AWESOME'
LowercaseProcessor: 'Rust is Awesome' -> 'rust is awesome'

=== Generic Programming with Containers ===
Integer container: 42
String container: "Hello, Rust!"
Mapped value (42 * 2): 84
Applied uppercase: HELLO, RUST!

=== Error Handling ===
Parsed number: 123
Expected error: parse error: failed to parse 'not a number': invalid digit found in string
Valid string: 'Rust'
```

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Course](https://github.com/rust-lang/rustlings/)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

## License

This project is provided as educational material.

## Contributing

Contributions are welcome! Please ensure:
- Code passes `cargo fmt`
- Clippy lints pass with no warnings
- All tests pass and coverage is maintained
- Commit messages are clear and descriptive
