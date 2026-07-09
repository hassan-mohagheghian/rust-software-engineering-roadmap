// Basic Functions
// ---
// Functions are the building blocks of Rust programs. Rust uses snake_case
// for function names and requires explicit parameter type annotations.
// Functions can return values, and the last expression is the return value.
//
// Key concepts:
// 1. Function declaration and calling
// 2. Parameters and return types
// 3. Expressions vs statements
// 4. Early return with `return`
// 5. Never return type (!)
// ---

fn main() {
    another_function(5, 'h');
    let result = add(3, 7);
    println!("3 + 7 = {result}");

    let greeting = greet("Rust");
    println!("{greeting}");

    // Statement vs Expression
    let y = {
        let x = 3;
        x + 1 // Expression (no semicolon = return value)
    };
    println!("y = {y}");

    // Early return
    let value = check_number(42);
    println!("Check result: {value}");

    // Never type
    // let _never: ! = panic!("unreachable");
}

fn another_function(x: i32, unit_label: char) {
    println!("The value is {x}{unit_label}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon = expression = return value
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn check_number(n: i32) -> &'static str {
    if n < 0 {
        return "negative"; // Early return
    }
    if n == 0 {
        return "zero";
    }
    "positive"
}
