// Scope and Shadowing
// ---
// Variables have a scope that starts where they are declared and ends at
// the closing brace. Shadowing allows you to re-declare a variable with
// the same name, which can even change its type.
//
// Key concepts:
// 1. Variable scope and lifetime
// 2. Shadowing vs mutation
// 3. Nested scopes
// 4. Scope and drop order
// 5. Block expressions
// ---

fn main() {
    // ==== Basic Scope ====
    let x = 10;
    {
        let y = 20;
        println!("Inner: x={x}, y={y}");
    }
    // println!("y={y}"); // Error: y is not in scope
    println!("Outer: x={x}");

    // ==== Shadowing vs Mutation ====
    let x = 5;
    let x = x + 1;     // Shadowing: creates a new variable
    {
        let x = x * 2;  // Shadowing in inner scope
        println!("Inner scope x: {x}");
    }
    println!("Outer scope x: {x}");

    // Shadowing can change type
    let data = "hello";
    let data = data.len(); // Now data is usize
    println!("Length: {data}");

    // ==== Nested Scopes ====
    let a = 1;
    let result = {
        let b = 2;
        let c = {
            let d = 3;
            a + b + d
        };
        c
    };
    println!("Nested result: {result}");

    // ==== Scope and Drop Order ====
    println!("=== Drop order ===");
    {
        let _a = DropTracker::new("first");
        let _b = DropTracker::new("second");
        let _c = DropTracker::new("third");
        println!("Inside scope");
    }
    println!("=== After scope ===");

    // ==== Block Expressions ====
    let y = {
        let x = 3;
        x + 1  // No semicolon = return value
    };
    println!("Block expression: {y}");

    // ==== Conditional Scope ====
    let show = true;
    let _name = if show {
        String::from("visible")
    } else {
        String::from("hidden")
    };

    // ==== Shadows in Loops ====
    let mut values = vec![1, 2, 3, 4, 5];
    for value in values.drain(..) {
        println!("Processing: {value}");
    }
    println!("After drain: {:?}", values);
}

struct DropTracker {
    name: String,
}

impl DropTracker {
    fn new(name: &str) -> Self {
        println!("  Created: {name}");
        DropTracker {
            name: name.to_string(),
        }
    }
}

impl Drop for DropTracker {
    fn drop(&mut self) {
        println!("  Dropped: {}", self.name);
    }
}
