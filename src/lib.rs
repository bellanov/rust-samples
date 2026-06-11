//! # Rust Samples
//!
//! A collection of modern Rust design patterns and examples.
//!
//! ## Features
//!
//! - Error handling with custom result types
//! - Trait-based abstractions
//! - Generic programming
//! - Zero-cost abstractions

use std::fmt;
use thiserror::Error;

/// Custom error type for library operations
#[derive(Error, Debug)]
pub enum RustSamplesError {
    /// Error when parsing fails
    #[error("parse error: {0}")]
    ParseError(String),

    /// Error when validation fails
    #[error("validation error: {0}")]
    ValidationError(String),

    /// Error for invalid input
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Result type for library operations
pub type Result<T> = std::result::Result<T, RustSamplesError>;

/// A trait demonstrating abstraction for data processing
pub trait DataProcessor {
    /// Process data and return a result
    fn process(&self, input: &str) -> Result<String>;

    /// Get the name of the processor
    fn name(&self) -> &str;
}

/// A simple text transformer that converts to uppercase
#[derive(Debug, Clone)]
pub struct UppercaseProcessor;

impl DataProcessor for UppercaseProcessor {
    fn process(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(RustSamplesError::InvalidInput(
                "input cannot be empty".to_string(),
            ));
        }
        Ok(input.to_uppercase())
    }

    fn name(&self) -> &str {
        "UppercaseProcessor"
    }
}

/// A simple text transformer that converts to lowercase
#[derive(Debug, Clone)]
pub struct LowercaseProcessor;

impl DataProcessor for LowercaseProcessor {
    fn process(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(RustSamplesError::InvalidInput(
                "input cannot be empty".to_string(),
            ));
        }
        Ok(input.to_lowercase())
    }

    fn name(&self) -> &str {
        "LowercaseProcessor"
    }
}

/// A generic value wrapper demonstrating generic programming
#[derive(Debug, Clone, PartialEq)]
pub struct Container<T: Clone + fmt::Debug> {
    value: T,
}

impl<T: Clone + fmt::Debug> Container<T> {
    /// Create a new container with a value
    pub fn new(value: T) -> Self {
        Container { value }
    }

    /// Get a reference to the contained value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Map the contained value to another type
    pub fn map<U: Clone + fmt::Debug, F: FnOnce(T) -> U>(self, f: F) -> Container<U> {
        Container::new(f(self.value))
    }

    /// Apply a function to the contained value
    pub fn apply<F: Fn(&T) -> String>(&self, f: F) -> String {
        f(&self.value)
    }
}

/// Parse an integer with error handling
pub fn parse_number(input: &str) -> Result<i32> {
    input
        .trim()
        .parse::<i32>()
        .map_err(|e| RustSamplesError::ParseError(format!("failed to parse '{}': {}", input, e)))
}

/// Validate that a string meets certain criteria
pub fn validate_string(input: &str, min_length: usize, max_length: usize) -> Result<&str> {
    let len = input.len();
    if len < min_length {
        return Err(RustSamplesError::ValidationError(format!(
            "string too short: {} < {}",
            len, min_length
        )));
    }
    if len > max_length {
        return Err(RustSamplesError::ValidationError(format!(
            "string too long: {} > {}",
            len, max_length
        )));
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_processor() {
        let processor = UppercaseProcessor;
        assert_eq!(processor.name(), "UppercaseProcessor");
        assert_eq!(processor.process("hello").unwrap(), "HELLO");
        assert_eq!(processor.process("Rust").unwrap(), "RUST");
    }

    #[test]
    fn test_uppercase_processor_empty_input() {
        let processor = UppercaseProcessor;
        assert!(processor.process("").is_err());
    }

    #[test]
    fn test_lowercase_processor() {
        let processor = LowercaseProcessor;
        assert_eq!(processor.name(), "LowercaseProcessor");
        assert_eq!(processor.process("HELLO").unwrap(), "hello");
        assert_eq!(processor.process("Rust").unwrap(), "rust");
    }

    #[test]
    fn test_lowercase_processor_empty_input() {
        let processor = LowercaseProcessor;
        assert!(processor.process("").is_err());
    }

    #[test]
    fn test_container_new_and_get() {
        let container = Container::new(42);
        assert_eq!(container.get(), &42);
    }

    #[test]
    fn test_container_map() {
        let container = Container::new(5);
        let mapped = container.map(|x| x * 2);
        assert_eq!(mapped.get(), &10);
    }

    #[test]
    fn test_container_apply() {
        let container = Container::new("hello");
        let result = container.apply(|s| s.to_uppercase());
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_container_equality() {
        let c1 = Container::new(42);
        let c2 = Container::new(42);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_parse_number_valid() {
        assert_eq!(parse_number("42").unwrap(), 42);
        assert_eq!(parse_number("-10").unwrap(), -10);
        assert_eq!(parse_number("  100  ").unwrap(), 100);
    }

    #[test]
    fn test_parse_number_invalid() {
        assert!(parse_number("not a number").is_err());
        assert!(parse_number("12.5").is_err());
        assert!(parse_number("").is_err());
    }

    #[test]
    fn test_validate_string_valid() {
        assert_eq!(validate_string("hello", 1, 10).unwrap(), "hello");
        assert_eq!(validate_string("x", 1, 1).unwrap(), "x");
    }

    #[test]
    fn test_validate_string_too_short() {
        assert!(validate_string("hi", 5, 10).is_err());
    }

    #[test]
    fn test_validate_string_too_long() {
        assert!(validate_string("hello world", 1, 5).is_err());
    }

    #[test]
    fn test_error_display() {
        let err = RustSamplesError::ParseError("bad input".to_string());
        assert_eq!(err.to_string(), "parse error: bad input");
    }
}
