// Hash Sets
// ---
// HashSet<T> stores unique values with O(1) lookup. It's implemented as
// a HashMap where the value is (). Useful for membership testing, set
// operations, and deduplication.
//
// Key concepts:
// 1. Creating hash sets
// 2. Inserting and checking membership
// 3. Set operations (union, intersection, difference)
// 4. Deduplication
// 5. Common patterns
// ---

use std::collections::HashSet;

fn main() {
    // ==== Creating Hash Sets ====
    let mut fruits = HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("cherry");
    println!("Fruits: {fruits:?}");

    // ==== From Vec (deduplication) ====
    let numbers = vec![1, 2, 3, 2, 1, 4, 3, 5];
    let unique: HashSet<_> = numbers.into_iter().collect();
    println!("Unique: {unique:?}");

    // ==== Checking Membership ====
    println!("Contains apple: {}", fruits.contains("apple"));
    println!("Contains orange: {}", fruits.contains("orange"));

    // ==== Set Operations ====
    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].into();
    let set_b: HashSet<i32> = [3, 4, 5, 6, 7].into();

    let union: HashSet<&i32> = set_a.union(&set_b).collect();
    println!("Union: {:?}", union);

    let intersection: HashSet<&i32> = set_a.intersection(&set_b).collect();
    println!("Intersection: {:?}", intersection);

    let difference: HashSet<&i32> = set_a.difference(&set_b).collect();
    println!("Difference (A-B): {:?}", difference);

    let symmetric_difference: HashSet<&i32> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric diff: {:?}", symmetric_difference);

    // ==== Subsets and Supersets ====
    let a: HashSet<i32> = [1, 2, 3].into();
    let b: HashSet<i32> = [1, 2, 3, 4, 5].into();
    println!("A is subset of B: {}", a.is_subset(&b));
    println!("B is superset of A: {}", b.is_superset(&a));

    // ==== Practical: Finding Common Elements ====
    let enrolled = HashSet::from(["Alice", "Bob", "Charlie"]);
    let completed = HashSet::from(["Bob", "Charlie", "Diana"]);
    let in_both: HashSet<&str> = enrolled.intersection(&completed).copied().collect();
    println!("Completed and enrolled: {in_both:?}");

    // ==== Disjoint Check ====
    let _empty: HashSet<i32> = HashSet::new();
    println!("Intersection is empty: {}", a.is_disjoint(&b));
}
