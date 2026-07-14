# Rust Software Engineering Examples

A practical collection of small Rust examples for learning software engineering
concepts step by step -- from zero to advanced.

The repository follows a learning path: Rust fundamentals, functions, control flow,
data structures, error handling, OOP patterns, generics, lifetimes, modules,
concurrency, traits, macros, testing, design patterns, system design, and
advanced topics -- then expands into the Rust ecosystem with tokio, axum, ratatui,
and more.

Each example is intentionally small and self-contained so it can be read, run,
and modified without needing a full application setup.

## Repository Structure

```
RSE_000_Rust_Basics/       Variables, types, ownership, borrowing, strings, scope
RSE_010_Functions/         Closures, iterators, function pointers, higher-order functions
RSE_020_Control_Flow/      Pattern matching, if/while let, loops, iterator adapters
RSE_030_Data_Structures/   Structs, enums, tuples, arrays, vectors, hashmaps, sets
RSE_040_Error_Handling/    Result, Option, panic, custom errors, the ? operator
RSE_050_OOP_in_Rust/       Encapsulation, trait objects, associated functions, methods
RSE_060_Generics/          Generic functions, structs, enums, trait bounds, where clauses
RSE_070_Lifetimes/         Lifetime annotations, elision rules, static lifetimes
RSE_080_Modules/           Module system, visibility, re-exports, workspaces
RSE_090_Crates/            External crates, dependency management, feature flags
RSE_100_Smart_Pointers/    Box, Rc, Arc, RefCell, Mutex, Cow
RSE_110_Concurrency/       Threads, channels, async/await, tokio, shared state
RSE_120_Traits/            Trait objects, default methods, supertraits, dispatch
RSE_130_Macros/            Declarative macros, derive macros, procedural macros
RSE_140_Testing/           Unit tests, integration tests, benchmarks, doc tests
RSE_150_Cargo/             Workspace, profiles, features, build scripts
RSE_160_Design_Patterns/   Builder, factory, observer, strategy, newtype
RSE_170_System_Design/     FFI, memory layout, networking, serialization
RSE_180_Ecosystem/         serde, tokio, reqwest, async patterns
RSE_190_Advanced/          Unsafe, advanced traits, typestate, const generics

--- Ecosystem & Frameworks (separate crates with real dependencies) ---

RSE_200_Tokio/             Async runtime: spawn, channels, timers, select, sync, shutdown
RSE_210_Serde/             Serialization: JSON, TOML, attributes, enums, custom impl
RSE_220_Clap/              CLI parsing: args, subcommands, validation, complex CLIs
RSE_230_Logging/           Tracing: structured logging, spans, filters, formats
RSE_240_HTTP_Client/       Reqwest: GET/POST, JSON, streaming, error handling
RSE_250_Database/          Sqlx: SQLite, CRUD, migrations, transactions, pools
RSE_260_Utilities/         Chrono + UUID: dates, time zones, unique identifiers
RSE_270_Middleware/        Tower + Tower-HTTP: service trait, layers, custom middleware
RSE_280_Axum/              Web framework: routing, JSON, state, errors, WebSocket, full API
RSE_290_Ratatui/           Terminal UI: widgets, layout, events, stateful, custom widgets
RSE_300_GUI/               Desktop GUI: egui basics, layouts, state, async integration
RSE_310_Capstone/          Combined project: TUI dashboard + Axum API + Tokio runtime
```

Folder names use this convention:
```
RSE_<NUMBER>_<TOPIC_NAME>
```
`RSE` means Rust Software Engineering. Numbers follow a learning path from
000 (beginner) to 310 (capstone project).

## How To Run

### Basic Examples (RSE_000 to RSE_190)

Run any example directly with Cargo:
```bash
cargo run --example RSE_000_Rust_Basics__RS_01_variables_and_mutability
```

### Ecosystem Examples (RSE_200 to RSE_310)

Each section is a separate workspace crate with its own dependencies:
```bash
# Run a specific example
cargo run -p rse-200-tokio --bin tokio_01_basics

# Run all examples in a section
cargo run -p rse-280-axum

# Check all crates compile
cargo check --workspace
```

### Examples by Category

**Async & Concurrency:**
```bash
cargo run -p rse-200-tokio --bin tokio_03_channels
cargo run -p rse-200-tokio --bin tokio_06_select
```

**Web Development:**
```bash
cargo run -p rse-280-axum --bin axum_08_full_api
cargo run -p rse-270-middleware --bin tower_http_02_custom
```

**Terminal UI:**
```bash
cargo run -p rse-290-ratatui --bin ratatui_07_advanced_app
```

**Desktop GUI:**
```bash
cargo run -p rse-300-gui
```

**Capstone Project:**
```bash
cargo run -p rse-310-capstone
```

## Requirements

- Rust >= 1.85 (edition 2024)
- Basic examples: no external dependencies
- Ecosystem examples: dependencies listed in each section's Cargo.toml

## Learning Path

1. **Fundamentals** (RSE_000 to RSE_030): Variables, functions, control flow, data structures
2. **Core Concepts** (RSE_040 to RSE_100): Error handling, OOP, generics, lifetimes, smart pointers
3. **Advanced Rust** (RSE_110 to RSE_190): Concurrency, traits, macros, testing, design patterns
4. **Async Runtime** (RSE_200): Tokio basics through advanced patterns
5. **Serialization** (RSE_210): Serde with JSON, TOML, custom implementations
6. **CLI Tools** (RSE_220): Clap for argument parsing and subcommands
7. **Observability** (RSE_230): Tracing for structured logging
8. **HTTP Client** (RSE_240): Reqwest for API calls and streaming
9. **Database** (RSE_250): Sqlx for async SQL with SQLite
10. **Utilities** (RSE_260): Chrono for dates, UUID for identifiers
11. **Middleware** (RSE_270): Tower and Tower-HTTP for composable middleware
12. **Web Framework** (RSE_280): Axum for building REST APIs
13. **Terminal UI** (RSE_290): Ratatui for building TUI applications
14. **Desktop GUI** (RSE_300): egui for building desktop applications
15. **Capstone** (RSE_310): Combined TUI + API application

## License

MIT License - Copyright 2026 Hassan Mohagheghian
