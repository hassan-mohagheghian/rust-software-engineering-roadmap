// Pattern Matching
// ---
// Pattern matching is one of Rust's most powerful features. The `match`
// expression compares a value against patterns and runs code for the
// matching pattern. It must be exhaustive (cover all possibilities).
//
// Key concepts:
// 1. Basic match syntax
// 2. Matching literals
// 3. Matching multiple patterns (|)
// 4. Matching ranges (..=)
// 5. Destructuring
// 6. Match guards
// 7. _ wildcard
// ---

fn main() {
    // ==== Basic Match ====
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // ==== Matching with Pipe ====
    let number = 4;
    match number {
        1 | 2 => println!("One or two"),
        3 | 4 => println!("Three or four"),
        _ => println!("Other"),
    }

    // ==== Matching Ranges ====
    let number = 15;
    match number {
        1..=5 => println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        11..=20 => println!("Between 11 and 20"),
        _ => println!("Something else"),
    }

    // ==== Destructuring Tuples ====
    let point = (3, -5);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {x}"),
        (0, y) => println!("On y-axis at {y}"),
        (x, y) => println!("At ({x}, {y})"),
    }

    // ==== Match Guards ====
    let number = 4;
    match number {
        n if n < 0 => println!("{n} is negative"),
        n if n == 0 => println!("zero"),
        n if n % 2 == 0 => println!("{n} is even"),
        n => println!("{n} is odd"),
    }

    // ==== Matching Enums ====
    let color = Color::Red;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Rgb(r, g, b) => println!("RGB({r}, {g}, {b})"),
        Color::Hsv(h, s, v) => println!("HSV({h}, {s}, {v})"),
    }

    // ==== Binding ====
    let message = Message::Quit;
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("Text: {text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Color: ({r}, {g}, {b})"),
        _ => println!("Other"),
    }

    // ==== Nested Destructuring ====
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: ({r}, {g}, {b})");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: ({h}, {s}, {v})");
        }
        _ => println!("Other message"),
    }
}

enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hsv(u16, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
