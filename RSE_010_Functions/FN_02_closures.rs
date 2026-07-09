// Closures
// ---
// Closures are anonymous functions that can capture variables from their
// enclosing scope. They are widely used in Rust for callbacks, iterators,
// and functional-style programming.
//
// Key concepts:
// 1. Closure syntax
// 2. Type inference in closures
// 3. Capturing environment (move, borrow, mutable borrow)
// 4. Closure as return type
// 5. Fn, FnMut, FnOnce traits
// ---

fn main() {
    // ==== Basic Closure ====
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // Short form (single expression)
    let double = |x| x * 2;
    println!("double(5) = {}", double(5));

    // ==== Capturing Environment ====

    // Immutable capture
    let name = String::from("Rust");
    let greeting = || format!("Hello, {name}!");
    println!("{}", greeting());

    // Mutable capture
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("Count: {}", increment());
    println!("Count: {}", increment());
    println!("Count: {}", increment());

    // Move closure (takes ownership)
    let name = String::from("Rust");
    let greeting = move || format!("Hello, {name}!");
    // println!("{name}"); // Error: name was moved
    println!("{}", greeting());

    // ==== Closure as Parameter ====
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {doubled:?}");

    // ==== Returning Closures ====
    let adder_10 = make_adder(10);
    println!("adder(5) = {}", adder_10(5));

    // ==== Fn Traits ====
    let closure = || println!("I'm a closure");
    call_with_fn(closure);

    let mut vec = vec![1, 2, 3];
    let mut mut_closure = || vec.push(4);
    call_with_fn_mut(&mut mut_closure);
    println!("Vec after push: {vec:?}");

    let name = String::from("world");
    let owned_closure = move || println!("Hello, {name}!");
    call_with_fn_once(owned_closure);
}

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn call_with_fn(f: impl Fn()) {
    f();
}

fn call_with_fn_mut(f: &mut impl FnMut()) {
    f();
}

fn call_with_fn_once(f: impl FnOnce()) {
    f();
}
