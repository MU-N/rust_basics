// Strings collection of characters. 
// Strings are UTF-8 encoded, represent languages and symbols.
// Strings are growable, mutable, owned, and stored as a sequence of bytes.
// Stored on the heap, allowing grow and shrink dynamically.
// Different from "&str' string slices, which are immutable 'view' of a string
// Created using 'String::new()' or 'String::from()' or 'to_string()' method on a string literal.
// Append using 'push_str()' to add a '&str' or 'push()' to add a 'char'.
// Concatenate using the '+' operator "Moves the ownership of the first string"


fn main() {
    let s = String::new();
    println!("Empty String: '{}'", s); // Print an empty string

    let mut s = String::from("Hello");
    println!("String from literal: '{}'", s); // Print the string created from a literal

    s.push_str(", world!"); // Append a string slice to the existing string
    println!("Appended String: '{}'", s); // Print the appended string

    s.push(' '); // Append a character to the existing string
    s.push_str("Welcome to Rust!"); // Append another string slice to the existing string
    println!("Final String: '{}'", s); // Print the final string

    let s1 = "Hello, ".to_string(); // Create a string using to_string() method on a string literal
    let s2 = "world!".to_string();
    let s3 = s1 + &s2; // Concatenate s1 and s2 using the '+' operator, which moves the ownership of s1
    //println!("s1: '{}'", s1); // This will cause a compile-time error because s1 has been moved and is no longer valid
    println!("s2: '{}'", s2); // Print s2, which is still valid
    println!("Concatenated String: '{}'", s3); // Print the concatenated string




}