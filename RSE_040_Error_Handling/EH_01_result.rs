// Result Type
// ---
// Result<T, E> is Rust's way of handling recoverable errors. It has two
// variants: Ok(T) for success and Err(E) for failure. The `?` operator
// provides ergonomic error propagation.
//
// Key concepts:
// 1. Result<T, E> type
// 2. Matching on Result
// 3. unwrap and expect
// 4. The ? operator for propagation
// 5. Converting between error types
// 6. Combining Results with combinators
// ---

use std::fs;

fn main() {
    // ==== Basic Result ====
    let result: Result<i32, String> = Ok(42);
    println!("Ok: {result:?}");

    let result: Result<i32, String> = Err(String::from("something went wrong"));
    println!("Err: {result:?}");

    // ==== Matching ====
    let numbers = vec!["1", "2", "three", "4"];
    for s in &numbers {
        match s.parse::<i32>() {
            Ok(n) => println!("Parsed: {n}"),
            Err(e) => println!("Failed to parse '{s}': {e}"),
        }
    }

    // ==== unwrap and expect ====
    let good: Result<i32, &str> = Ok(42);
    println!("unwrap: {}", good.unwrap());
    println!("expect: {}", good.expect("should be ok"));

    // let bad: Result<i32, &str> = Err("oops");
    // bad.unwrap(); // Panics!
    // bad.expect("this will panic with message"); // Better panic message

    // ==== ? Operator ====
    match read_number_from_file("numbers.txt") {
        Ok(n) => println!("Read number: {n}"),
        Err(e) => println!("Error: {e}"),
    }

    // ==== Combinators ====
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|x| x * 2);
    println!("Doubled: {doubled:?}");

    let result: Result<i32, &str> = Err("oops");
    let value = result.unwrap_or(0);
    println!("Default: {value}");

    let result: Result<i32, &str> = Ok(5);
    let value = result.unwrap_or_else(|_| {
        println!("Computing default...");
        42
    });
    println!("Default: {value}");

    // ==== Chaining Combinators ====
    let result = Ok::<i32, &str>(5)
        .map(|x| x + 1)
        .map(|x| x * 2)
        .and_then(|x| if x > 0 { Ok(x) } else { Err("negative") });
    println!("Chained: {result:?}");

    // ==== Collecting Results ====
    let strings = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse()).collect();
    println!("All parsed: {numbers:?}");

    let strings = vec!["1", "two", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse()).collect();
    println!("With error: {numbers:?}");
}

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let number = content.trim().parse::<i32>()?;
    Ok(number)
}
