// While Let Expressions
// ---
// `while let` runs a loop as long as a pattern matches. It's especially
// useful for processing data structures like stacks, queues, and iterators
// where you want to keep going until a pattern fails.
//
// Key concepts:
// 1. Basic while let loop
// 2. Processing with pop()
// 3. Iterator-like patterns
// 4. While let vs loop+match
// 5. Practical use cases
// ---

fn main() {
    // ==== Basic while let ====
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        println!("Popped: {top}");
    }
    println!("Stack is empty: {:?}", stack);

    // ==== Processing a Queue ====
    let mut queue = vec!["task1", "task2", "task3"];
    while let Some(task) = queue.pop() {
        println!("Processing: {task}");
    }

    // ==== While let with Option ====
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.drain(..);
    while let Some(num) = iter.next() {
        print!("{num} ");
    }
    println!();

    // ==== Nested while let ====
    let mut outer = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    while let Some(mut inner) = outer.pop() {
        while let Some(val) = inner.pop() {
            print!("{val} ");
        }
        println!();
    }

    // ==== While let vs loop+match ====
    let mut data = Some(0);
    // Using while let (cleaner)
    while let Some(val) = data {
        println!("Value: {val}");
        data = if val < 3 { Some(val + 1) } else { None };
    }

    // Equivalent with loop+match (more verbose)
    loop {
        match data {
            Some(val) => {
                println!("Value: {val}");
                data = if val < 3 { Some(val + 1) } else { None };
            }
            None => break,
        }
    }
}
