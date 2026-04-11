// Functions are a fundamental building block of Rust programs. They allow you to group related code together and reuse it throughout your program.
// A function is defined using the fn keyword, followed by the function name, a list of parameters in parentheses, and a block of code in curly braces.
// The syntax for defining a function is as follows:
// fn function_name(parameter1: type1, parameter2: type2, ...) -> return_type {
//     // function body
// }

// no parameters and no return value
fn greet() {
    println!("Hello, world!");
}

// taking parameters
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Here is an example of a simple function that takes two parameters and returns their sum:
fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn main() {
    greet(); // Call the greet function to print "Hello, world!" to the console

    greet_person("Alice"); // Call the greet_person function with the argument "Alice" to print "Hello, Alice!" to the console

    let result = add(5, 3); // result is 8
    println!("The sum of 5 and 3 is: {}", result); // The sum of 5 and 3 is: 8
}
