// Tuples
// ---
// Tuples group values of different types into a fixed-size compound type.
// They support destructuring, indexing, and can be used as function return
// values to return multiple values.
//
// Key concepts:
// 1. Tuple creation and types
// 2. Destructuring
// 3. Tuple indexing
// 4. Nested tuples
// 5. Unit type ()
// 6. Returning tuples from functions
// ---

fn main() {
    // ==== Creating Tuples ====
    let tup: (i32, f64, bool) = (500, 6.4, true);
    println!("Tuple: {:?}", tup);

    // ==== Destructuring ====
    let (x, y, z) = tup;
    println!("x={x}, y={y}, z={z}");

    // ==== Indexing ====
    println!("First: {}", tup.0);
    println!("Second: {}", tup.1);
    println!("Third: {}", tup.2);

    // ==== Nested Tuples ====
    let nested = ((1, 2), (3, 4));
    println!("Nested: {:?}", nested);
    println!("Inner: {:?}", nested.0);
    println!("Value: {}", (nested.0).1);

    // ==== Unit Type ====
    let unit: () = ();
    println!("Unit: {:?}", unit);

    // ==== Returning Multiple Values ====
    let (min, max) = min_max(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Min: {min}, Max: {max}");

    // ==== Tuple as Function Parameter ====
    let point = (3.0, 4.0);
    let distance = distance_from_origin(point);
    println!("Distance from origin: {distance}");

    // ==== Comparing Tuples ====
    let a = (1, 2);
    let b = (1, 2);
    let c = (1, 3);
    println!("a == b: {}", a == b);
    println!("a == c: {}", a == c);
    println!("a < c: {}", a < c);
}

fn min_max(v: Vec<i32>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &val in &v {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }
    (min, max)
}

fn distance_from_origin(point: (f64, f64)) -> f64 {
    (point.0.powi(2) + point.1.powi(2)).sqrt()
}
