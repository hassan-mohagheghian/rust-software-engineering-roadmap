/// Error Handling - The ? Operator
///
/// The `?` operator is syntactic sugar for early-return-on-error. It calls
/// `From::from()` to convert error types and returns `Err` automatically.
/// It chains beautifully and makes error propagation clean and idiomatic.
///
/// Key concepts:
/// 1. ? operator basics — unwrap Ok, propagate Err
/// 2. Type conversion with From trait
/// 3. Chaining multiple ? in one function
/// 4. ? in closures and combinators
/// 5. Early returns vs nested match
/// ---

use std::fs;
use std::io;
use std::num::ParseIntError;

// ==== Simple chain with ? ====

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?; // io::Error
    let number = content.trim().parse::<i32>()?; // ParseIntError
    Ok(number)
}

// ==== Converting error types with ? ====

#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Parse(ParseIntError),
    MissingField(String),
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::Parse(e)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "IO error: {e}"),
            ConfigError::Parse(e) => write!(f, "Parse error: {e}"),
            ConfigError::MissingField(field) => write!(f, "Missing field: {field}"),
        }
    }
}

fn read_config(path: &str) -> Result<(String, u16), ConfigError> {
    let content = fs::read_to_string(path)?; // io::Error -> ConfigError
    let mut name = None;
    let mut port = None;

    for line in content.lines() {
        if let Some(val) = line.strip_prefix("name=") {
            name = Some(val.to_string());
        }
        if let Some(val) = line.strip_prefix("port=") {
            port = Some(val.parse::<u16>()?); // ParseIntError -> ConfigError
        }
    }

    let name = name.ok_or(ConfigError::MissingField("name".to_string()))?;
    let port = port.ok_or(ConfigError::MissingField("port".to_string()))?;

    Ok((name, port))
}

// ==== ? vs match comparison ====

fn with_match(input: &str) -> Result<i32, String> {
    let parsed = match input.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };
    Ok(parsed + 10)
}

fn with_question_mark(input: &str) -> Result<i32, String> {
    let parsed = input.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(parsed + 10)
}

fn main() {
    // Basic ? chaining
    println!("=== Basic ? Chaining ===");
    match read_number_from_file("nonexistent.txt") {
        Ok(n) => println!("Number: {n}"),
        Err(e) => println!("Error: {e}"),
    }

    // ? with From conversion
    println!("\n=== ? with Type Conversion ===");
    match read_config("config.txt") {
        Ok((name, port)) => println!("Config: name={name}, port={port}"),
        Err(e) => println!("Config error: {e}"),
    }

    // ? vs match comparison
    println!("\n=== ? vs match ===");
    let m = with_match("42");
    let q = with_question_mark("42");
    println!("match result: {m:?}");
    println!("? result: {q:?}");

    let m = with_match("abc");
    let q = with_question_mark("abc");
    println!("match error: {m:?}");
    println!("? error: {q:?}");

    // Chaining many ? in one function
    println!("\n=== Deep Chain ===");
    fn deep_chain(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let first_line = content.lines().next().ok_or("Empty file")?;
        let number: i32 = first_line.parse()?;
        Ok(format!("First line parsed to: {number}"))
    }

    match deep_chain("nonexistent.txt") {
        Ok(s) => println!("{s}"),
        Err(e) => println!("Deep chain error: {e}"),
    }
}
