/// Error Handling - Panic and Unwrap
///
/// Not every error is recoverable. `panic!` halts the program when something
/// goes fundamentally wrong — invariant violations, unreachable states, or
/// programmer mistakes. Understanding when to panic vs return Result is key.
///
/// Key concepts:
/// 1. panic! macro for unrecoverable errors
/// 2. unwrap() and expect() — convenience with panic on failure
/// 3. When to panic vs return Result
/// 4. catch_unwind for recovery
/// 5. Backtraces and debugging panics
/// ---
use std::panic;

// ==== When to panic (invariant violation) ====

fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        panic!("Cannot calculate average of empty slice — this is a programmer error");
    }
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

// ==== When NOT to panic (use Result instead) ====

fn parse_and_double(input: &str) -> Result<i32, String> {
    let n = input
        .parse::<i32>()
        .map_err(|e| format!("Invalid number: {e}"))?;
    Ok(n * 2)
}

// ==== catch_unwind for recoverable panics ====

fn might_panic(input: &str) -> String {
    if input.is_empty() {
        panic!("Empty input!");
    }
    format!("Processed: {input}")
}

fn main() {
    // unwrap — panics on None/Err
    println!("=== Unwrap ===");
    let some_value: Option<i32> = Some(42);
    println!("Some(42).unwrap() = {}", some_value.unwrap());

    let none_value: Option<i32> = None;
    // This would panic: none_value.unwrap();
    println!("None would panic if unwrapped — skipping demo");

    // expect — unwrap with a custom message
    println!("\n=== Expect ===");
    let path = std::env::current_dir().expect("Should be able to get current dir");
    println!("Current dir: {path:?}");

    // When to panic: programmer errors / invariant violations
    println!("\n=== When to Panic ===");
    let avg = calculate_average(&[1.0, 2.0, 3.0]);
    println!("Average: {avg}");
    // calculate_average(&[]) would panic — caller's bug

    // When to use Result: external/invalid input
    println!("\n=== When to Use Result ===");
    for input in ["21", "abc", ""] {
        match parse_and_double(input) {
            Ok(v) => println!("\"{input}\" -> {v}"),
            Err(e) => println!("\"{input}\" -> Error: {e}"),
        }
    }

    // catch_unwind — catch panics without crashing
    println!("\n=== catch_unwind ===");
    let result = panic::catch_unwind(|| might_panic("hello"));
    println!("Success: {result:?}");

    let result = panic::catch_unwind(|| might_panic(""));
    match result {
        Ok(v) => println!("Got: {v}"),
        Err(_) => println!("Caught a panic without crashing!"),
    }

    // Rule of thumb summary
    println!("\n=== Rules of Thumb ===");
    println!("panic! when:");
    println!("  - An invariant is violated (empty slice, index out of bounds)");
    println!("  - A test assertion fails");
    println!("  - Setting up something that should never fails");
    println!("Result when:");
    println!("  - Input might be invalid (user input, file I/O, network)");
    println!("  - The caller can reasonably recover");
    println!("  - Failure is expected as a normal outcome");
}
