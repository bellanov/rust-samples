//! Example binary demonstrating the rust-samples library

use rust_samples::{Container, DataProcessor, LowercaseProcessor, UppercaseProcessor};

fn main() {
    println!("Rust Samples - Modern Design Patterns Demo\n");

    // Demonstrate trait-based abstraction
    demonstrate_processors();

    // Demonstrate generic programming
    demonstrate_containers();

    // Demonstrate error handling
    demonstrate_error_handling();
}

fn demonstrate_processors() {
    println!("=== Trait-Based Abstraction ===");

    let processors: Vec<Box<dyn DataProcessor>> =
        vec![Box::new(UppercaseProcessor), Box::new(LowercaseProcessor)];

    let input = "Rust is Awesome";

    for processor in &processors {
        match processor.process(input) {
            Ok(output) => println!("{}: '{}' -> '{}'", processor.name(), input, output),
            Err(e) => eprintln!("Error in {}: {}", processor.name(), e),
        }
    }
    println!();
}

fn demonstrate_containers() {
    println!("=== Generic Programming with Containers ===");

    let int_container = Container::new(42);
    println!("Integer container: {:?}", int_container.get());

    let string_container = Container::new("Hello, Rust!");
    println!("String container: {:?}", string_container.get());

    let mapped = int_container.clone().map(|x| x * 2);
    println!("Mapped value (42 * 2): {:?}", mapped.get());

    let result = string_container.apply(|s| s.to_uppercase());
    println!("Applied uppercase: {}", result);
    println!();
}

fn demonstrate_error_handling() {
    println!("=== Error Handling ===");

    // Valid parse
    match rust_samples::parse_number("123") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Invalid parse
    match rust_samples::parse_number("not a number") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Expected error: {}", e),
    }

    // Validation
    match rust_samples::validate_string("Rust", 2, 10) {
        Ok(s) => println!("Valid string: '{}'", s),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!();
}
