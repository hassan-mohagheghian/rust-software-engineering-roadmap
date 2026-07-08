// Scalar Types
// ---
// Rust has four primary scalar types: integers, floating-point numbers,
// booleans, and characters. Each has specific sizes and ranges that you
// can choose based on your needs.
//
// Key concepts:
// 1. Integer types (i8, i16, i32, i64, i128, isize, u8..usize)
// 2. Floating-point types (f32, f64)
// 3. Boolean type (bool)
// 4. Character type (char)
// 5. Type suffixes and literals
// ---

fn main() {
    // ==== Integer Types ====
    let a: i8 = -128; // signed 8-bit
    let b: u8 = 255; // unsigned 8-bit
    let c: i16 = -32_768; // signed 16-bit
    let d: u16 = 65_535; // unsigned 16-bit
    let e: i32 = -2_147_483_648; // signed 32-bit
    let f: u32 = 4_294_967_295; // unsigned 32-bit
    let g: i64 = -9_223_372_036_854_775_808;
    let h: u64 = 18_446_744_073_709_551_615;
    let i: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let j: usize = 42; // pointer-sized unsigned

    println!("i8: {a}, u8: {b}, i16: {c}, u16: {d}");
    println!("i32: {e}, u32: {f}, i64: {g}, u64: {h}");
    println!("i128: {i}, usize: {j}");

    // ==== Integer Literals ====
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("Decimal: {decimal}, Hex: {hex}, Octal: {octal}");
    println!("Binary: {binary}, Byte: {byte}");

    // ==== Floating-Point Types ====
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32

    println!("f64: {x}, f32: {y}");

    // Floating-point special values
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;

    println!("Infinity: {inf}, -Infinity: {neg_inf}, NaN: {nan}");

    // ==== Boolean Type ====
    let t = true;
    let f: bool = false;

    println!("True: {t}, False: {f}");

    // ==== Character Type ====
    let c = 'z';
    let z: char = 'ℤ';
    let heart = '❤';
    let cat = '🐈';

    println!("Chars: {c}, {z}, {heart}, {cat}");
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}
