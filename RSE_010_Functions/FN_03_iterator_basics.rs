// Iterator Basics
// ---
// Iterators are a core abstraction in Rust. The Iterator trait provides a
// uniform way to process sequences of values. Iterators are lazy - they
// do nothing until consumed.
//
// Key concepts:
// 1. Iterator trait and next()
// 2. Creating iterators (iter, into_iter, iter_mut)
// 3. Consuming iterators (collect, sum, count)
// 4. Lazy evaluation
// 5. Creating custom iterators
// ---

fn main() {
    // ==== Creating Iterators ====
    let v = vec![1, 2, 3];

    // iter() - borrows elements
    for val in v.iter() {
        print!("{val} ");
    }
    println!();

    // into_iter() - takes ownership
    for val in v.into_iter() {
        print!("{val} ");
    }
    println!();
    // println!("{v:?}"); // Error: v was consumed

    // ==== Consuming Iterators ====
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    println!("Sum: {sum}");

    let count = v.iter().count();
    println!("Count: {count}");

    let max = v.iter().max();
    println!("Max: {:?}", max);

    // ==== Iterator Adapters (Lazy) ====
    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Doubled: {doubled:?}");

    let even: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even: {even:?}");

    // Chaining adapters
    let result: Vec<i32> = v.iter().filter(|&&x| x > 2).map(|&x| x * 10).collect();
    println!("Filtered and mapped: {result:?}");

    // ==== Custom Iterator ====
    let counter = Counter::new(5);
    let values: Vec<u32> = counter.collect();
    println!("Counter: {values:?}");
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
