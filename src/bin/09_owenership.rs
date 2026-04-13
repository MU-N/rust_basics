// Ownership is a fundamental concept in Rust that governs how memory is managed.
// It is based on three main rules:
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped and the memory will be freed.
// These rules ensure that Rust can manage memory safely and efficiently without the need for a garbage collector

// owenership perpouses
// Ownership is used to manage memory in Rust. It allows Rust to automatically deallocate memory when

// what is stored on the stack and what is stored on the heap
// In Rust, values can be stored on either the stack or the heap, depending on their size and lifetime.
// Values that are stored on the stack are typically small and have a fixed size, such as integers, floating-point numbers, booleans, and characters. These values are stored directly in memory and are automatically deallocated when they go out of scope.
// Values that are stored on the heap are typically larger and have a variable size, such as strings, vectors, and other complex data structures. These values are stored in a separate area of memory called the heap, and they are accessed through a pointer. When a value is stored on the heap, Rust automatically manages the memory for that value, ensuring that it is deallocated when it is no longer needed.

fn main() {
    // stack variable copied when assigned to another variable
    let x: i32 = 5; // x is an integer variable stored on the stack
    let y: i32 = x; // y is a copy of x, both x and y are stored on the stack
    println!("x = {}, ", x); // x = 5
    println!("y = {}, ", y); // y = 5

    // heap variable moved when assigned to another variable
    let s1: String = String::from("Hello"); // s1 is a string variable stored on the heap
    let mut s2: String = s1; // s2 is a copy of s1
    //println!("s1 = {}, ", s1); // s1 is moved to s2, s1 is no longer valid and cannot be used
    println!("s2 = {}, ", s2); // s2 = Hello
    println!("-------------------------------");

    // move, copy, and clone
    // When we assign a stack variable to another variable, Rust creates a copy of the value and both variables can be used independently.
    // However, when we assign a heap variable to another variable, Rust moves the ownership of the value from the original variable to the new variable. This means that the original variable is no longer valid and cannot be used after the move.
    // If we want to create a copy of a heap variable instead of moving it, we can use the clone method, which creates a deep copy of the value on the heap. This allows us to have two separate variables that own their own memory on the heap.
    // we can also use the clone method to create a deep copy of a heap variable, which allows us to have two separate variables that own their own memory on the heap.
    // Be careful when using the clone method, as it can be expensive in terms of performance if the value being cloned is large or complex. It is generally recommended to use cloning only when necessary and to consider other options, such as borrowing or using references, to avoid unnecessary cloning.
    let s3: String = s2.clone(); // s3 is a deep copy of
    println!("s2 = {}, ", s2); // s2 = Hello    
    println!("s3 = {}, ", s3); // s3 = Hello
    println!("-------------------------------");

    s2.push_str(", world!"); // s2 is modified, but s3 is not affected because it is a deep copy
    println!("s2 = {}, ", s2); // s2 = Hello, world!
    println!("s3 = {}, ", s3); // s3 = Hello
    println!("-------------------------------");


    // Functions and ownership
    // When we pass a variable to a function, Rust moves the ownership of the value from the caller to the function. 
    // This means that the caller can no longer use the variable after it has been passed to the function.
    // If we want to allow the caller to use the variable after it has been passed to the function, we can use references, which allow us to borrow the value without taking ownership of it.

    let s4: String = String::from("Hello"); // s4 is a string variable stored on the heap
    takes_ownership(s4); // s4 is moved to the function, s4 is no longer valid and cannot be used after this point
    //println!("s4 = {}, ", s4); // s4 is no longer valid


    let s5: String = String::from("Hello"); // s5 is a string variable stored on the heap
    takes_ownership(s5.clone()); // s5 is cloned and the clone is moved to
    // the function, s5 is still valid and can be used after this point
    println!("s5 = {}, ", s5); // s5 = Hello


    let s6: String = String::from("Hello"); // s6 is a string variable stored on the heap   
    take_reference(&s6); // s6 is passed as a reference to the function, s6 is still valid and can be used after this point
    println!("s6 = {}, ", s6); // s6 = Hello


}

fn takes_ownership(s: String) {
    println!("s takes ownership = {}, ", s); // s = Hello
    // s goes out of scope at the end of this function, and the memory for s is automatically deallocated
}

fn take_reference(s: &String) {
    println!("s takes reference = {}, ", s); // s = Hello
    // s is a reference to a string variable, and it does not take ownership of the value. 
    // The caller can still use the original variable after this function is called.
}
