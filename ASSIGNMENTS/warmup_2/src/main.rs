// Problem #2: Clone and Modify
// Given a string, clone it and modify the cloned string by appending a new word. Print both the original string and the cloned, modified string to show that the original has not been changed.

fn clone_and_modify(s: &String) -> String {
    let mut changed = s.clone();
    changed.push_str("World!");

    return changed;
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}