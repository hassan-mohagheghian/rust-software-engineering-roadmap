// If Expressions
// ---
// In Rust, `if` is an expression that returns a value. This eliminates the
// need for ternary operators found in other languages. The condition must
// be a bool, and both branches must return the same type.
//
// Key concepts:
// 1. if as an expression
// 2. if-else chains
// 3. if-else if-else chains
// 4. let-if expressions
// 5. Condition must be bool (no truthy/falsy)
// ---

fn main() {
    // ==== Basic if ====
    let number = 7;
    if number > 5 {
        println!("Greater than 5");
    }

    // ==== if-else ====
    let number = 3;
    if number % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    // ==== if-else if-else ====
    let number = 6;
    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Even");
    } else {
        println!("None of the above");
    }

    // ==== if as Expression ====
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Value: {value}");

    // ==== let-if ====
    let name = "Rust";
    let greeting = if name == "Rust" {
        "Hello, Rustacean!"
    } else {
        "Hello, stranger!"
    };
    println!("{greeting}");

    // ==== Nested if ====
    let x = 10;
    let y = 20;
    if x > 5 {
        if y > 15 {
            println!("Both conditions met");
        }
    }

    // ==== Boolean Conditions ====
    let is_active = true;
    let has_permission = true;
    if is_active && has_permission {
        println!("Access granted");
    }

    if !false {
        println!("This always runs");
    }
}
