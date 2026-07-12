// If Let Expressions
// ---
// `if let` is a shorthand for matching a single pattern. It's useful when
// you only care about one case and want to destructure a value.
// `let else` provides early return on pattern mismatch.
//
// Key concepts:
// 1. if let syntax
// 2. if let with else
// 3. let else for early returns
// 4. Chaining if let
// 5. Comparing with match
// ---

fn main() {
    // ==== Basic if let ====
    let config_max: Option<u32> = Some(3);
    if let Some(max) = config_max {
        println!("Maximum is configured to be {max}");
    }

    // ==== if let with else ====
    let favorite_color: Option<&str> = None;
    if let Some(color) = favorite_color {
        println!("Your favorite color is {color}");
    } else {
        println!("No favorite color set");
    }

    // ==== let else ====
    let name = get_name();
    let Some(name) = name else {
        println!("No name provided");
        return;
    };
    println!("Hello, {name}!");

    // ==== Matching Enum Variants ====
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        println!("Not a state quarter");
    }

    // ==== Chaining if let ====
    let value: Option<i32> = Some(42);
    if let Some(v) = value {
        if v > 0 {
            println!("Positive value: {v}");
        } else {
            println!("Non-positive value: {v}");
        }
    }

    // ==== Let Else Pattern ====
    fn process(input: Option<&str>) -> String {
        let Some(text) = input else {
            return String::from("no input");
        };
        text.to_uppercase()
    }
    println!("Process: {}", process(Some("hello")));
    println!("Process: {}", process(None));
}

fn get_name() -> Option<&'static str> {
    Some("Rustacean")
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
