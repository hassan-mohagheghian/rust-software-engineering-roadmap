// Loop Expressions
// ---
// Rust has three loop constructs: `loop` (infinite), `while` (conditional),
// and `for` (iterator-based). The `loop` expression can return a value
// using `break` with an expression.
//
// Key concepts:
// 1. Infinite loop with loop
// 2. break with return value
// 3. Labels for nested loops
// 4. for loops with ranges
// 5. for with iterators
// 6. loop vs while
// ---

fn main() {
    // ==== Basic Loop ====
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Loop result: {result}");

    // ==== Labeled Breaks (Nested Loops) ====
    let mut count = 0;
    'outer: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("  remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Final count: {count}");

    // ==== For Loops ====
    let numbers = [10, 20, 30, 40, 50];
    for element in numbers {
        println!("Value: {element}");
    }

    // ==== Range Loops ====
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    for i in (1..5).rev() {
        print!("{i} ");
    }
    println!();

    // ==== For with enumerate ====
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{index}: {fruit}");
    }

    // ==== While Loop ====
    let mut n = 1;
    while n < 1000 {
        n *= 2;
    }
    println!("Final n: {n}");

    // ==== Loop with Return ====
    let input = "42";
    let parsed: Option<i32> = loop {
        match input.parse::<i32>() {
            Ok(n) => break Some(n),
            Err(_) => break None,
        };
    };
    println!("Parsed: {parsed:?}");
}
