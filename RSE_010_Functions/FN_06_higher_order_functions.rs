// Higher-Order Functions
// ---
// Higher-order functions are functions that take other functions as arguments
// or return functions as results. Rust's iterator adapter pattern is a
// powerful example of higher-order function usage.
//
// Key concepts:
// 1. map, filter, fold
// 2. Function composition
// 3. Returning closures from functions
// 4. Currying-like patterns
// 5. Real-world iterator chains
// ---

fn main() {
    // ==== Map, Filter, Fold ====
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum_of_squares_of_evens: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("Sum of squares of evens: {sum_of_squares_of_evens}");

    // ==== Fold (Reduce) ====
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {product}");

    let concatenated: String =
        ["Hello", " ", "World", "!"]
            .iter()
            .fold(String::new(), |mut acc, &s| {
                acc.push_str(s);
                acc
            });
    println!("Concatenated: {concatenated}");

    // ==== Function Composition ====
    let add_ten = |x| x + 10;
    let double = |x| x * 2;
    let composed = compose(double, add_ten);
    println!("compose(double, add_ten)(5) = {}", composed(5));

    // ==== Returning Closures ====
    let multiplier = make_multiplier(3);
    let values: Vec<i32> = (1..=5).map(multiplier).collect();
    println!("Multiplied: {values:?}");

    // ==== Currying Pattern ====
    let add = make_adder(10);
    println!("add(5) = {}", add(5));
    println!("add(20) = {}", add(20));

    // ==== Real-World Iterator Chain ====
    let words = vec!["hello", "world", "rust", "is", "great"];
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect();
    println!("Filtered and upper-cased: {result:?}");

    // ==== enumerate, zip, take ====
    let names = vec!["Alice", "Bob", "Charlie"];
    for (i, name) in names.iter().enumerate() {
        println!("  {i}: {name}");
    }

    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let sum: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    println!("Zip sum: {sum:?}");

    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("First three: {first_three:?}");
}

fn compose(f: impl Fn(i32) -> i32, g: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(g(x))
}

fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
