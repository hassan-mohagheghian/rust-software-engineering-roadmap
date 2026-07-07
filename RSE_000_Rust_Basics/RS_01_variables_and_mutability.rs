// Variables and Mutability
// ---
// Variables in Rust are immutable by default. To change a value, you must
// declare it with `mut`. Rust uses static typing with type inference, so
// you often don't need to annotate types explicitly.
//
// Key concepts:
// 1. Immutable variables (default)
// 2. Mutable variables with `mut`
// 3. Variable shadowing
// 4. Type inference
// 5. Const declarations
// ---

fn main() {
    // ==== Immutable Variables ====
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // This would cause a compile error

    // ==== Mutable Variables ====
    let mut y = 10;
    println!("The value of y is: {y}");
    y = 20;
    println!("The value of y is now: {y}");

    // ==== Type Inference ====
    let name = "Rust"; // &str inferred
    let age: u32 = 25; // explicit type annotation
    let pi = 3.14; // f64 inferred
    let is_active = true; // bool inferred

    println!("Name: {name}, Age: {age}, Pi: {pi}, Active: {is_active}");

    // ==== Shadowing ====
    let z = 5;
    let z = z + 1; // new variable shadows the old one
    let z = z * 2; // shadow again
    println!("The value of z is: {z}");

    // Shadowing can also change the type
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize
    println!("Number of spaces: {spaces}");

    // ==== Const Declarations ====
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159265358979;
    println!("Max points: {MAX_POINTS}, PI: {PI}");

    // ==== Underscores for Readability ====
    let million = 1_000_000;
    let hex = 0xff;
    let binary = 0b1111_0000;
    println!("Million: {million}, Hex: {hex}, Binary: {binary}");
}
