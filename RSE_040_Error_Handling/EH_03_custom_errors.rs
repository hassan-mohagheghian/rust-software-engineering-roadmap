/// Error Handling - Custom Error Types
///
/// Real applications need structured error types. Rust lets you define your own
/// error enums that implement `std::error::Error`. This gives callers rich
/// information about what went wrong and enables matching on specific variants.
///
/// Key concepts:
/// 1. Defining error enums with `thiserror` or manual impl
/// 2. Implementing std::error::Error and Display
/// 3. From trait for automatic conversion
/// 4. Box<dyn Error> for dynamic dispatch errors
/// 5. Error context and chaining
/// ---
use std::fmt;
use std::num::ParseIntError;

// ==== Manual error type ====

#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(ParseIntError),
    PermissionDenied,
    RateLimited { retry_after_secs: u64 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "Not found: {name}"),
            AppError::ParseError(e) => write!(f, "Parse error: {e}"),
            AppError::PermissionDenied => write!(f, "Permission denied"),
            AppError::RateLimited { retry_after_secs } => {
                write!(f, "Rate limited, retry after {retry_after_secs}s")
            }
        }
    }
}

impl std::error::Error for AppError {}

// From<ParseIntError> enables ? to auto-convert parse errors
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// ==== Functions returning custom errors ====

fn get_user_role(user_id: u32) -> Result<String, AppError> {
    match user_id {
        1 => Ok("admin".to_string()),
        2 => Ok("user".to_string()),
        0 => Err(AppError::PermissionDenied),
        _ => Err(AppError::NotFound(format!("user {user_id}"))),
    }
}

fn parse_config_value(input: &str) -> Result<u32, AppError> {
    let value: u32 = input.parse()?; // From<ParseIntError> converts automatically
    if value > 1000 {
        Err(AppError::RateLimited {
            retry_after_secs: 60,
        })
    } else {
        Ok(value)
    }
}

// ==== Error mapping ====

fn transform_error(input: &str) -> Result<u32, String> {
    parse_config_value(input).map_err(|e| format!("Config error: {e}"))
}

fn main() {
    // Pattern matching on error variants
    println!("=== Custom Error Variants ===");
    for id in [1, 2, 0, 99] {
        match get_user_role(id) {
            Ok(role) => println!("User {id}: {role}"),
            Err(AppError::PermissionDenied) => println!("User {id}: no access"),
            Err(AppError::NotFound(name)) => println!("User {id}: {name}"),
            Err(e) => println!("User {id}: unexpected error: {e}"),
        }
    }

    // Parse errors with ? operator
    println!("\n=== Parse with ? ===");
    for input in ["50", "abc", "2000"] {
        match parse_config_value(input) {
            Ok(v) => println!("\"{input}\" -> {v}"),
            Err(e) => println!("\"{input}\" -> {e}"),
        }
    }

    // Error mapping
    println!("\n=== Error Mapping ===");
    match transform_error("42") {
        Ok(v) => println!("transform_error(\"42\") -> {v}"),
        Err(e) => println!("transform_error(\"42\") -> {e}"),
    }
    match transform_error("bad") {
        Ok(v) => println!("transform_error(\"bad\") -> {v}"),
        Err(e) => println!("transform_error(\"bad\") -> {e}"),
    }

    // Box<dyn Error> for type-erased errors
    println!("\n=== Box<dyn Error> ===");
    fn dynamic_error(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let v: u32 = input.parse()?;
        Ok(v)
    }
    match dynamic_error("42") {
        Ok(v) => println!("Dynamic: {v}"),
        Err(e) => println!("Dynamic error: {e}"),
    }
}
