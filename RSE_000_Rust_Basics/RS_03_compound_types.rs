// Compound Types
// ---
// Rust has two compound types: tuples and arrays. Tuples group values of
// different types together, while arrays group values of the same type.
// Slices provide a view into contiguous sequences.
//
// Key concepts:
// 1. Tuple creation and destructuring
// 2. Tuple indexing
// 3. Unit type (empty tuple)
// 4. Array creation and access
// 5. Array slicing
// 6. Multi-dimensional arrays
// ---

fn main() {
    // ==== Tuples ====
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);

    // Destructuring
    let (x, y, z) = tup;
    println!("Destructured: x={x}, y={y}, z={z}");

    // Indexing
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("Indexed: first={first}, second={second}, third={third}");

    // Unit type (empty tuple)
    let unit: () = ();
    println!("Unit type: {:?}", unit);

    // ==== Arrays ====
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [0; 10]; // 10 zeros

    println!("Array: {:?}", arr);
    println!("Typed array: {:?}", arr2);
    println!("Zeros array: {:?}", arr3);

    // Accessing elements
    let first = arr[0];
    let second = arr[1];
    println!("First: {first}, Second: {second}");

    // Array length
    println!("Length: {}", arr.len());

    // ==== Slices ====
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // elements at index 1 and 2
    let all = &a[..];      // entire array
    let from_start = &a[..3]; // first 3 elements
    let to_end = &a[2..];  // from index 2 to end

    println!("Slice [1..3]: {:?}", slice);
    println!("All: {:?}", all);
    println!("First 3: {:?}", from_start);
    println!("From index 2: {:?}", to_end);

    // ==== Multi-dimensional Arrays ====
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("Matrix: {:?}", matrix);
    println!("Element [1][2]: {}", matrix[1][2]);

    // ==== Iterating Over Arrays ====
    for element in arr {
        print!("{element} ");
    }
    println!();
}
