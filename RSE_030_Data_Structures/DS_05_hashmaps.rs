// Hash Maps
// ---
// HashMap<K, V> stores key-value pairs with O(1) average lookup. Unlike
// vectors, hash maps are not included in the prelude, so you must import
// them explicitly.
//
// Key concepts:
// 1. Creating hash maps
// 2. Inserting and accessing
// 3. Entry API (or_insert, or_insert_with)
// 4. Iterating
// 5. Updating strategies
// ---

use std::collections::HashMap;

fn main() {
    // ==== Creating Hash Maps ====
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("Scores: {scores:?}");

    // ==== From Tuples ====
    let teams = vec![
        (String::from("Blue"), 10),
        (String::from("Red"), 50),
    ];
    let scores: HashMap<_, _> = teams.into_iter().collect();
    println!("From tuples: {scores:?}");

    // ==== Accessing Values ====
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("Blue's score: {score}"),
        None => println!("Team not found"),
    }

    // ==== Entry API ====
    let mut map = HashMap::new();
    map.entry(String::from("a")).or_insert(1);
    map.entry(String::from("a")).or_insert(2); // Won't overwrite
    println!("Entry API: {map:?}");

    // Counting occurrences
    let text = "hello world wonderful world hello";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {word_count:?}");

    // ==== Updating ====
    let mut map = HashMap::new();
    map.insert("key", "value");
    map.insert("key", "new_value"); // Overwrites
    println!("Updated: {map:?}");

    // ==== Iterating ====
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // ==== Common Operations ====
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    println!("Len: {}", map.len());
    println!("Contains 'a': {}", map.contains_key("a"));
    println!("Remove 'b': {:?}", map.remove("b"));
    println!("After remove: {map:?}");

    // ==== HashMap of Vectors ====
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    departments.entry("Engineering".to_string())
        .or_insert_with(Vec::new)
        .push("Alice".to_string());
    departments.entry("Engineering".to_string())
        .or_insert_with(Vec::new)
        .push("Bob".to_string());
    departments.entry("Marketing".to_string())
        .or_insert_with(Vec::new)
        .push("Charlie".to_string());
    println!("Departments: {departments:#?}");
}
