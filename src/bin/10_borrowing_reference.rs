
// Borrowing and References
// In Rust, borrowing and references are fundamental concepts that allow us to manage memory safely and efficiently. 
//Borrowing allows us to temporarily use a value without taking ownership of it, 
// while references allow us to create pointers to values without taking ownership. 
// This helps prevent issues like dangling pointers and data races, and it allows us to write safe and efficient code without the need for a garbage collector.

// Rules of borrowing and references
// 1. At any given time, you can have either one mutable reference or any number of immutable references to a value, but not both.
// 2. References must always be valid. This means that a reference cannot outlive the value it points to, and it cannot be used after the value has been dropped.
// 3. Mutable references allow you to modify the value they point to, while immutable references do not allow you to modify the value they point to. 
// This means that if you have a mutable reference to a value, you cannot have any immutable references to that value at the same time, and vice versa.


fn main()
{
    let s1: String = String::from("Hello"); // s1 is a string variable stored on the heap
    let len: usize = calculate_length(&s1); // Pass a reference to s1 to the calculate_length function
    println!("The length of '{}' is {}.", s1, len); // The length of 'Hello' is 5. s1 is still valid and can be used after the function call because we passed a reference to it, rather than moving ownership of it to the function.

    let mut s2: String = String::from("Hello"); // s2 is a mutable string variable stored on the heap
    let len: usize = calculate_length_mut(&mut s2); // Pass a mutable reference
    println!("The length of '{}' is {}.", s2, len); // The length of 'Hello world!' is 13. s2 is still valid and can be used after the function call because we passed a mutable reference to it, rather than moving ownership of it to the function.


    // we can also have multiple immutable references to the same value at the same time, but we cannot have a mutable reference to that value at the same time. 
    // This is because mutable references allow us to modify the value they point to, and having multiple mutable references to the same value could
    // lead to data races and other memory safety issues.
    let mut s3: String = String::from("Hello"); // s3 is a string variable stored on the heap
    let r1: &String = &s3; // r1 is an immutable reference
    let r2: &String = &s3; // r2 is another immutable reference
    println!("r1 = {}, r2 = {}", r1, r2); // r1 = Hello, r2 = Hello


    let r1: &mut String = &mut s3; // r1 is a mutable reference to s3
    // let r2: &mut String = &mut s3; // This line will cause a compile-time error because we cannot have two mutable references to the same value at the same time. To fix this, we can either remove the second mutable reference or change it to an immutable reference.
    println!("r1 = {}", r1); // r1 = Hello
}

fn calculate_length(s: &String) -> usize {
    // s.add_str(" world!"); // This line will cause a compile-time error because s is an immutable reference and cannot be modified. To fix this, we can change the parameter type to &mut String and pass a mutable reference to the function.
    s.len() // Return the length of the string that s points to
}

fn calculate_length_mut(s: &mut String) -> usize {
    s.push_str(" world!"); // This line will modify the string that s points to because s is a mutable reference. We can now call this function with a mutable reference to a string variable.
    s.len() // Return the length of the modified string that s points to
}

// Dangling references
// A dangling reference is a reference that points to a value that has been dropped or deallocated
/*fn dangling_reference( ) -> &String {
    let s: String = String::from("Hello"); // s is a string variable stored on the heap
    &s // This line will cause a compile-time error because we are returning a reference to a value that will be dropped when the function ends. To fix this, we can return the string itself instead of a reference to it, or we can return a reference to a value that has a longer lifetime than the function.
}*/