// Option Type
// ---
// Option<T> represents an optional value: either Some(T) or None. It
// replaces null/nil from other languages with a type-safe alternative
// that forces you to handle the absence case.
//
// Key concepts:
// 1. Option<T> type
// 2. Some and None variants
// 3. Unwrapping options safely
// 4. Option combinators (map, and_then, unwrap_or)
// 5. Option and Result interplay
// ---

fn main() {
    // ==== Basic Option ====
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
    println!("some_number: {some_number:?}");
    println!("no_number: {no_number:?}");

    // ==== Matching ====
    match some_number {
        Some(n) => println!("Got: {n}"),
        None => println!("Nothing"),
    }

    // ==== unwrap_or and unwrap_or_else ====
    let value: Option<i32> = None;
    let value = value.unwrap_or(0);
    println!("Default: {value}");

    let value: Option<i32> = None;
    let value = value.unwrap_or_else(|| {
        println!("Computing default...");
        42
    });
    println!("Default: {value}");

    // ==== Map ====
    let some_string = Some("42");
    let some_number: Option<i32> = some_string.map(|s| s.parse().unwrap());
    println!("Mapped: {some_number:?}");

    // ==== and_then (Flatmap) ====
    fn double_if_even(x: i32) -> Option<i32> {
        if x % 2 == 0 { Some(x * 2) } else { None }
    }
    let result = Some(4).and_then(double_if_even);
    println!("and_then: {result:?}");

    let result = Some(3).and_then(double_if_even);
    println!("and_then (odd): {result:?}");

    // ==== Filtering ====
    let result = Some(42).filter(|&x| x > 10);
    println!("Filter: {result:?}");

    let result = Some(5).filter(|&x| x > 10);
    println!("Filter (too small): {result:?}");

    // ==== Option and Result ====
    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some(String::from("Alice")),
            2 => Some(String::from("Bob")),
            _ => None,
        }
    }

    let user = find_user(1).unwrap_or(String::from("Unknown"));
    println!("User: {user}");

    // ==== Collecting Options ====
    let numbers = vec![Some(1), None, Some(3)];
    let collected: Option<Vec<i32>> = numbers.into_iter().collect();
    println!("Collected: {collected:?}");

    // ==== Option in Iterators ====
    let names = vec!["Alice", "", "Bob", "", "Charlie"];
    let non_empty: Vec<&str> = names.iter().filter(|s| !s.is_empty()).copied().collect();
    println!("Non-empty: {non_empty:?}");
}
