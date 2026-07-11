// Function Pointers
// ---
// Rust supports function pointers (fn) which allow passing functions as
// arguments and storing them in data structures. Unlike closures, function
// pointers don't capture their environment.
//
// Key concepts:
// 1. fn type (function pointers)
// 2. Passing functions as arguments
// 3. Function pointers in structs
// 4. Comparing function pointers
// 5. Difference from closures
// ---

fn main() {
    // ==== Basic Function Pointer ====
    let f: fn(i32) -> i32 = double;
    println!("double(5) = {}", f(5));

    // ==== Passing Functions as Arguments ====
    let numbers = vec![1, 2, 3, 4, 5];
    let results = apply_to_vec(&numbers, double);
    println!("Doubled: {results:?}");

    let results = apply_to_vec(&numbers, square);
    println!("Squared: {results:?}");

    // ==== Function Pointers in Structs ====
    let calc = Calculator { operation: add };
    println!("3 + 4 = {}", calc.compute(3, 4));

    // ==== Function Array ====
    let operations: Vec<fn(i32, i32) -> i32> = vec![add, subtract, multiply];
    let a = 10;
    let b = 5;
    for op in &operations {
        println!("Result: {}", op(a, b));
    }

    // ==== Closures vs Function Pointers ====
    // 1. Closure that captures environment
    let offset = 10;

    let add_offset = |x: i32| x + offset; // captures `offset`

    let r = add_offset(30);
    println!("Closure with capture add_offset(30): {r}");

    // A capturing closure cannot be converted to a function pointer,
    // because a function pointer has no place to store captured data.
    //
    // let captured_fn: fn(i32) -> i32 = add_offset; // ❌ Error

    // 2. Normal function can be stored as a function pointer
    let double_fn: fn(i32) -> i32 = double;

    println!("Function pointer double_fn(5): {}", double_fn(5));

    // 3. Closure without capture can be converted to a function pointer

    let add_two = |x: i32| x + 2; // does NOT capture anything

    let add_two_fn: fn(i32) -> i32 = add_two; // ✅ closure coerces to fn pointer

    let r = add_two_fn(10);
    println!("Non-capturing closure as fn pointer add_two_fn(10): {r}");

    // ==== Comparing Function Pointers ====
    let f1: fn(i32) -> i32 = double;
    let f2: fn(i32) -> i32 = double;
    println!("Same function: {}", f1 as usize == f2 as usize);
}

fn double(x: i32) -> i32 {
    x * 2
}
fn square(x: i32) -> i32 {
    x * x
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn apply_to_vec(v: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    v.iter().map(|&x| f(x)).collect()
}

struct Calculator {
    operation: fn(i32, i32) -> i32,
}

impl Calculator {
    fn compute(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}
