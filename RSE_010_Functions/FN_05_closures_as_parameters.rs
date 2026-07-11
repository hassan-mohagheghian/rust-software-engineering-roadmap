// Closures as Parameters
// ---
// Closures can be passed as function parameters using generic type bounds
// with Fn, FnMut, or FnOnce traits. This enables flexible, composable code
// patterns similar to higher-order functions in functional programming.
//
// Key concepts:
// 1. Fn trait bound for immutable closures
// 2. FnMut trait bound for mutable closures
// 3. FnOnce trait bound for consuming closures
// 4. Multiple closure parameters
// 5. Generic vs impl Trait syntax
// ---

fn main() {
    // ==== Fn Trait Bound ====
    let v = vec![1, 2, 3, 4, 5];
    let result = apply(&v, |x| x * 2);
    println!("Doubled: {result:?}");

    let result = apply(&v, |x| x + 10);
    println!("Plus 10: {result:?}");

    // ==== FnMut Trait Bound ====
    let mut total = 0;
    accumulate(&v, |x| total += x);
    println!("Total: {total}");

    // ==== FnOnce Trait Bound ====
    let name = String::from("Rust");
    consume_and_print(name, |s| println!("Consumed: {s}"));

    // ==== Multiple Closures ====
    let result = apply_two(&v, |x| x * 2, |x| x + 1);
    println!("Double then add 1: {result:?}");

    // ==== Generic Syntax ====
    let result = apply_generic(&v, |x| x * 2);
    println!("Generic apply: {result:?}");

    // ==== Real-World Pattern: sort_by ====
    let mut names = vec!["Charlie", "Alice", "Bob"];
    names.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("Sorted by length: {names:?}");

    names.sort_by(|a, b| a.cmp(b));
    println!("Sorted alphabetically: {names:?}");
}

fn apply(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    v.iter().map(|&x| f(x)).collect()
}

fn accumulate(v: &[i32], mut f: impl FnMut(i32)) {
    for &x in v {
        f(x);
    }
}

fn consume_and_print(s: String, f: impl FnOnce(String)) {
    f(s);
}

fn apply_two<T: Fn(i32) -> i32, U: Fn(i32) -> i32>(v: &[i32], f: T, g: U) -> Vec<i32> {
    v.iter().map(|&x| g(f(x))).collect()
}

fn apply_generic<T>(v: &[i32], f: T) -> Vec<i32>
where
    T: Fn(i32) -> i32,
{
    v.iter().map(|&x| f(x)).collect()
}
