// Borrowing and References
// ---
// Instead of transferring ownership, you can create references that borrow
// a value. References allow you to access data without taking ownership.
// Rust enforces rules at compile time to prevent data races.
//
// Key concepts:
// 1. Immutable references (&T) - multiple allowed
// 2. Mutable references (&mut T) - only one at a time
// 3. Cannot have both simultaneously
// 4. Reference lifetime within scope
// 5. Dereferencing with *
// ---

fn main() {
    // ==== Immutable References ====
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}.");

    // Multiple immutable references are allowed
    let r1 = &s;
    let r2 = &s;
    println!("r1: {r1}, r2: {r2}");

    // ==== Mutable References ====
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed: {s}");

    // ==== Restriction: One Mutable Reference ====
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once
    r1.push_str(" world");
    println!("r1: {r1}");

    // ==== Mixing Immutable and Mutable (Non-overlapping) ====
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {r1}, r2: {r2}");
    // r1 and r2 are no longer used after this point
    let r3 = &mut s;
    r3.push_str("!");
    println!("r3: {r3}");

    // ==== Dereferencing ====
    let x = 5;
    let y = &x;
    assert_eq!(5, *y); // Dereference y to get the value
    println!("Dereferenced: {}", *y);

    // ==== Reference Slicing ====
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // Borrow elements 1, 2, 3
    println!("Slice: {:?}", slice);

    // ==== Dangling References (Prevented by Compiler) ====
    // let reference_to_nothing = dangle(); // This would not compile

    // ==== Passing References to Functions ====
    let data = vec![1, 2, 3];
    process_data(&data);
    println!("Original data: {:?}", data);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn process_data(data: &[i32]) {
    // Can read but not modify
    let sum: i32 = data.iter().sum();
    println!("Sum of data: {sum}");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
