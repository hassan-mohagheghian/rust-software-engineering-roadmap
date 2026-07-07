# Rust Software Engineering Examples

A practical collection of small Rust examples for learning software engineering
concepts step by step -- from zero to advanced.

The repository follows a learning path: Rust fundamentals, functions, control flow,
data structures, error handling, OOP patterns, generics, lifetimes, modules,
concurrency, traits, macros, testing, design patterns, system design, and
advanced topics.

Each example is intentionally small and self-contained so it can be read, run,
and modified without needing a full application setup.

## Repository Structure

```
src/examples/
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
```

Folder names use this convention:
```
RSE_<NUMBER>_<TOPIC_NAME>
```
`RSE` means Rust Software Engineering. Numbers follow a learning path from
000 (beginner) to 190 (advanced), with gaps for future expansion.

## How To Run

Run any example directly with Cargo:
```bash
cargo run --example RSE_000_Rust_Basics__RS_01_variables_and_mutability
```

Or run all examples:
```bash
cargo run --examples
```

Run the test suite:
```bash
cargo test
```

## Requirements

- Rust >= 1.85 (edition 2024)
- No external dependencies required (only stdlib used in most examples)

## License

MIT License - Copyright 2026 Hassan Mohagheghian
