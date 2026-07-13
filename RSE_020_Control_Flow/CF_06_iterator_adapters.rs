// Iterator Adapters
// ---
// Iterator adapters transform iterators into other iterators. They are
// lazy (don't execute until consumed) and can be chained for complex
// data transformations. This is Rust's functional programming powerhouse.
//
// Key concepts:
// 1. map, filter, enumerate
// 2. zip, chain, take, skip
// 3. fold, reduce, collect
// 4. flat_map, filter_map
// 5. peekable iterators
// ---

fn main() {
    // ==== map ====
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {doubled:?}");

    // ==== filter ====
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {evens:?}");

    // ==== enumerate ====
    let names = vec!["Alice", "Bob", "Charlie"];
    for (i, name) in names.iter().enumerate() {
        println!("  {i}: {name}");
    }

    // ==== zip ====
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let results: Vec<(&str, i32)> = names.iter().copied().zip(scores.iter().copied()).collect();
    println!("Results: {results:?}");

    // ==== chain ====
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let combined: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("Combined: {combined:?}");

    // ==== take and skip ====
    let numbers: Vec<i32> = (1..=10).collect();
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_five: Vec<&i32> = numbers.iter().skip(5).collect();
    println!("First 3: {first_three:?}");
    println!("Skip 5: {skip_five:?}");

    // ==== fold ====
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {sum}");

    // ==== flat_map ====
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
    println!("Chars: {:?}", chars);

    // ==== filter_map ====
    let strings = vec!["1", "two", "3", "four", "5"];
    let numbers: Vec<i32> = strings.iter().filter_map(|s| s.parse().ok()).collect();
    println!("Parsed numbers: {numbers:?}");

    // ==== peekable ====
    let mut iter = vec![1, 2, 3].into_iter().peekable();
    println!("Peek: {:?}", iter.peek());
    println!("Next: {:?}", iter.next());
    println!("Peek: {:?}", iter.peek());

    // ==== Chaining Multiple Adapters ====
    let result: Vec<String> = (1..=20)
        .filter(|x| x % 3 == 0)
        .map(|x| format!("num_{x}"))
        .take(3)
        .collect();
    println!("Chained result: {result:?}");
}
