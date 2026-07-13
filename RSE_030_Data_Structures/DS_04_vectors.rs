// Vectors
// ---
// Vectors (Vec<T>) are growable, heap-allocated arrays. They are the most
// commonly used collection in Rust, providing dynamic sizing with O(1)
// indexing and amortized O(1) push.
//
// Key concepts:
// 1. Creating vectors
// 2. Adding and removing elements
// 3. Accessing elements (index vs get)
// 4. Iterating
// 5. Vector of enums (heterogeneous lists)
// 6. Common operations
// ---

fn main() {
    // ==== Creating Vectors ====
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let v = vec![0; 10]; // 10 zeros
    println!("Vector: {:?}", v);

    // ==== Adding Elements ====
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After push: {v:?}");

    // ==== Accessing Elements ====
    let v = vec![1, 2, 3, 4, 5];
    println!("First: {}", v[0]);
    println!("Third: {}", v[2]);

    // Safe access with get()
    match v.get(100) {
        Some(val) => println!("Value: {val}"),
        None => println!("Index out of bounds"),
    }

    // ==== Removing Elements ====
    let mut v = vec![1, 2, 3, 4, 5];
    v.pop();          // Remove last
    println!("After pop: {v:?}");

    v.remove(0);      // Remove by index
    println!("After remove: {v:?}");

    v.retain(|&x| x > 2); // Keep only > 2
    println!("After retain: {v:?}");

    // ==== Iterating ====
    let v = vec![10, 20, 30];
    for val in &v {
        print!("{val} ");
    }
    println!();

    for val in &mut v.clone() {
        *val += 100;
    }

    // ==== Vector of Enums (Heterogeneous) ====
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(4.5),
        SpreadsheetCell::Text(String::from("hello")),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => print!("{i} "),
            SpreadsheetCell::Float(f) => print!("{f} "),
            SpreadsheetCell::Text(s) => print!("{s} "),
        }
    }
    println!();

    // ==== Common Operations ====
    let v = vec![1, 2, 3, 4, 5];
    println!("Len: {}", v.len());
    println!("Is empty: {}", v.is_empty());
    println!("Contains 3: {}", v.contains(&3));
    println!("Sum: {}", v.iter().sum::<i32>());
    println!("Sorted: {:?}", {
        let mut v = v.clone();
        v.sort();
        v
    });
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
