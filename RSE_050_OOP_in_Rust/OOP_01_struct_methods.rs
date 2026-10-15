// Struct Methods and Associated Functions
// ---
// Rust implements OOP concepts through structs, enums, and traits rather
// than traditional class-based inheritance. Methods are defined in impl
// blocks, and associated functions act like static methods.
//
// Key concepts:
// 1. Method definitions with &self, &mut self, self
// 2. Associated functions (static methods)
// 3. Multiple impl blocks
// 4. Self type
// 5. Method resolution
// ---

fn main() {
    // ==== Creating Instances ====
    let mut counter = Counter::new();
    println!("Initial count: {}", counter.get());

    counter.increment();
    counter.increment();
    counter.increment();
    println!("After incrementing: {}", counter.get());

    counter.reset();
    println!("After reset: {}", counter.get());

    // ==== Associated Functions ====
    let c1 = Counter::from_value(10);
    let c2 = Counter::from_range(1, 5);
    println!("c1: {}", c1.get());
    println!("c2: {}", c2.get());

    // ==== Builder Pattern ====
    let config = Config::builder()
        .host("localhost")
        .port(8080)
        .max_connections(100)
        .build();
    println!("Config: {config:?}");

    // ==== Method Chaining ====
    let result = Calculator::new()
        .add(10.0)
        .multiply(2.0)
        .subtract(5.0)
        .get_result();
    println!("(10 * 2) - 5 = {result}");
}

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn from_value(v: i32) -> Self {
        Counter { value: v }
    }

    fn from_range(start: i32, end: i32) -> Self {
        Counter { value: end - start }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn get(&self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    max_connections: u32,
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder {
            host: None,
            port: None,
            max_connections: None,
        }
    }
}

struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    max_connections: Option<u32>,
}

impl ConfigBuilder {
    fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = Some(max);
        self
    }

    fn build(self) -> Config {
        Config {
            host: self.host.unwrap_or_else(|| "0.0.0.0".to_string()),
            port: self.port.unwrap_or(3000),
            max_connections: self.max_connections.unwrap_or(50),
        }
    }
}

struct Calculator {
    value: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator { value: 0.0 }
    }

    fn add(mut self, n: f64) -> Self {
        self.value += n;
        self
    }

    fn multiply(mut self, n: f64) -> Self {
        self.value *= n;
        self
    }

    fn subtract(mut self, n: f64) -> Self {
        self.value -= n;
        self
    }

    fn get_result(self) -> f64 {
        self.value
    }
}
