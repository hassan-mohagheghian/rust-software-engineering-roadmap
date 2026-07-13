// Enums
// ---
// Enums define types by enumerating possible variants. Each variant can
// hold different data. Enums combined with pattern matching are one of
// Rust's most powerful features.
//
// Key concepts:
// 1. Basic enum definition
// 2. Enums with data
// 3. Option<T> (the null substitute)
// 4. Methods on enums
// 5. Representing IP addresses, shapes, etc.
// ---

fn main() {
    // ==== Basic Enum ====
    let direction = Direction::North;
    match direction {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }

    // ==== Enums with Data ====
    let msg = Message::Write(String::from("Hello"));
    process_message(msg);

    let msg = Message::Move { x: 10, y: 20 };
    process_message(msg);

    let msg = Message::Color(255, 128, 0);
    process_message(msg);

    let msg = Message::Quit;
    process_message(msg);

    // ==== Option<T> ====
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // Unwrapping Option
    match some_number {
        Some(n) => println!("Got number: {n}"),
        None => println!("No number"),
    }

    // ==== Methods on Enums ====
    let shape = Shape::Circle(5.0);
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
}

enum Direction {
    North,
    South,
    East,
    West,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("Text: {text}"),
        Message::Color(r, g, b) => println!("Color: ({r}, {g}, {b})"),
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * (w + h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}
