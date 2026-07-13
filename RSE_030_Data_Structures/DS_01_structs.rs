// Structs
// ---
// Structs are custom data types that group related values together.
// Unlike tuples, each field has a name, making the data self-documenting.
// Rust supports structs with methods and associated functions.
//
// Key concepts:
// 1. Defining structs
// 2. Creating instances
// 3. Field init shorthand
// 4. Struct update syntax (..)
// 5. Tuple structs and unit structs
// 6. Methods with impl blocks
// ---

fn main() {
    // ==== Basic Struct ====
    let user = User {
        username: String::from("rustacean"),
        email: String::from("rust@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {}, Email: {}", user.username, user.email);

    // ==== Mutable Struct ====
    let mut user = User {
        username: String::from("rustacean"),
        email: String::from("rust@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user.email = String::from("new@example.com");
    println!("Updated email: {}", user.email);

    // ==== Field Init Shorthand ====
    let username = String::from("admin");
    let email = String::from("admin@example.com");
    let user = User {
        username, // shorthand for username: username
        email,
        active: true,
        sign_in_count: 1,
    };
    println!("User: {}", user.username);

    // ==== Struct Update Syntax ====
    let user2 = User {
        username: String::from("admin2"),
        ..user // fill remaining fields from `user`
    };
    println!("User2: {}", user2.username);

    // ==== Tuple Struct ====
    let black = Color(0, 0, 0);
    let _white = Color(255, 255, 255);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);

    // ==== Unit Struct ====
    let _always_equal = AlwaysEqual;

    // ==== Methods ====
    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());

    // ==== Associated Functions ====
    let sq = Rectangle::square(5.0);
    println!("Square area: {}", sq.area());
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct AlwaysEqual;

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
