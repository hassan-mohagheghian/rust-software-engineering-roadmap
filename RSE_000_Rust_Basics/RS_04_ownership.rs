// Ownership
// ---
// Ownership is Rust's most unique feature. Every value has exactly one owner,
// and when the owner goes out of scope, the value is dropped (freed). This
// eliminates the need for a garbage collector while ensuring memory safety.
//
// Key concepts:
// 1. Ownership rules (single owner, move on transfer)
// 2. Move semantics for heap-allocated data
// 3. Clone for explicit deep copies
// 4. Copy trait for stack-only types
// 5. Drop and RAII
// ---

fn main() {
    // ==== Ownership Rules ====
    {
        let s = String::from("hello"); // s comes into scope
        println!("String: {s}");
    } // s goes out of scope, memory is freed

    // ==== Move Semantics ====
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{s1}"); // This would cause a compile error!
    println!("s2: {s2}");

    // ==== Clone for Deep Copy ====
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicit deep copy
    println!("s1: {s1}, s2: {s2}"); // Both are valid

    // ==== Copy Trait (Stack-only Types) ====
    let x = 5;
    let y = x; // Copy, not move (i32 implements Copy)
    println!("x: {x}, y: {y}"); // Both are valid

    let a = true;
    let b = a; // bool implements Copy
    println!("a: {a}, b: {b}");

    // ==== Ownership and Functions ====
    let name = String::from("Rust");
    greet(name); // name is moved into the function
    // println!("{name}"); // Error: value used after move

    let number = 42;
    double(number); // number is copied (i32 implements Copy)
    println!("Number still valid: {number}");

    // ==== Returning Ownership ====
    let s1 = gives_ownership();
    println!("Got: {s1}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved, then returned
    println!("Returned: {s3}");

    // ==== Drop and RAII ====
    println!("--- Creating resource ---");
    let _resource = Resource::new("database");
    println!("--- Using resource ---");
    println!("--- End of scope, resource will be dropped ---");
}

fn greet(name: String) {
    println!("Hello, {name}!");
}

fn double(x: i32) {
    println!("Double: {}", x * 2);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s // Return the string back to the caller
}

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("  Resource '{}' created", name);
        Resource {
            name: name.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  Resource '{}' dropped", self.name);
    }
}
