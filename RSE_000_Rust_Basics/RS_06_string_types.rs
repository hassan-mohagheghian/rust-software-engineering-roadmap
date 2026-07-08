// String Types
// ---
// Rust has two main string types: String (heap-allocated, growable) and
// &str (string slice, borrowed). Understanding the difference is crucial
// for writing idiomatic Rust code.
//
// Key concepts:
// 1. String vs &str
// 2. String creation methods
// 3. String manipulation
// 4. String slicing and indexing
// 5. Converting between String and &str
// 6. UTF-8 encoding
// ---

fn main() {
    // ==== Creating Strings ====
    let s1 = String::from("hello");
    let s2 = "world".to_string();
    let s3 = String::new();
    let s4 = format!("{} {}!", s1, s2);

    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3 (empty): '{s3}'");
    println!("s4 (format): {s4}");

    // ==== String Manipulation ====
    let mut greeting = String::from("Hello");
    greeting.push(' ');
    greeting.push_str("World");
    println!("greeting: {greeting}");

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    println!("s3: {s3}");

    // Format macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");

    // ==== String Slicing ====
    let hello = String::from("Hello, world!");
    let hello_slice = &hello[0..5];
    println!("Slice: {hello_slice}");

    // ==== String and &str Conversion ====
    let s: &str = "hello"; // &str (string literal)
    let s_owned: String = s.to_string(); // Convert &str to String
    let s_ref: &str = &s_owned; // Convert String to &str

    println!("&str: {s}");
    println!("String: {s_owned}");
    println!("&str from String: {s_ref}");

    // ==== UTF-8 Strings ====
    let hello = String::from("مرحبا"); // Arabic
    let hello2 = String::from("你好"); // Chinese
    let hello3 = String::from("🌍"); // Emoji

    println!("Arabic: {hello}");
    println!("Chinese: {hello2}");
    println!("Emoji: {hello3}");

    // Bytes vs chars vs graphemes
    println!("Bytes: {}", hello3.len());
    println!("Chars: {}", hello3.chars().count());
    // use unicode_segmentation::UnicodeSegmentation;
    // println!("Graphemes: {}", hello3.graphemes(true).count());

    // ==== Useful Methods ====
    let s = "  Hello, World!  ";
    println!("Trimmed: '{}'", s.trim());
    println!("Uppercase: {}", "hello".to_uppercase());
    println!("Contains 'World': {}", s.contains("World"));
    println!("Replace: {}", "hello world".replace("world", "Rust"));
    println!("Split: {:?}", "a,b,c".split(',').collect::<Vec<_>>());
}
