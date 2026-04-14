// Rust has no nulls and instead uses the Option enum to represent a value that can either be present (Some) or absent (None).
// This allows Rust to provide safety guarantees and prevent null pointer exceptions, which are a common source of bugs in other programming languages.
// it uses two enums, 'Option<T>' and 'Result<T, E>', to handle cases where a value may be absent or an error may occur.
// Option<T> for value that can either be present (Some) or absent (None).
// - Some(T) value is present
// - None value is absent
// Result<T, E> for operations that can either succeed (Ok) or fail (Err), where T is the type of the successful value and E is the type of the error value.
// - Ok(T) operation succeeded with a value of type T
// - Err(E) operation failed with an error of type E

// Match is the standard way to handle the different variants of an enum in Rust.
// .unwrap() is a a shortcut method that can get the value or panic
// .expect("custom error message") is a method that can get the value or panic with a custom error message
// The ?  operator can propagate errors in a concise way.

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide by zero!"); // This will cause a runtime error and terminate the program with a message indicating that division by zero is not allowed.
    }
    a / b // Return the result of dividing a by b
}

fn divide_result(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero!")) // Return an error value of type String indicating that division by zero is not allowed
    } else {
        Ok(a / b) // Return a successful value of type f64 representing the result of dividing a by b
    }
}

fn main() {
    let result = divide_result(10.0, 2.0); // Call the divide_result function with a = 10.0 and b = 2.0, which should return Ok(5.0)
    match result {
        Ok(value) => println!("The result of division is: {}", value), // Handle the Ok variant and print the successful value
        Err(e) => println!("Error: {}", e), // Handle the Err variant and print the error message
    }

    let result = divide_result(10.0, 0.0); // Call the divide_result function with a = 10.0 and b = 0.0, which should return Err("Cannot divide by zero!")
    match result {
        Ok(value) => println!("The result of division is: {}", value), // Handle the Ok variant and print the successful value
        Err(e) => println!("Error: {}", e), // Handle the Err variant and print the error message
    }

    // Using .unwrap() to get the value or panic
    let good_result = divide_result(10.0, 0.2).unwrap();
    println!("The result of division is: {}", good_result);

    // Using .expect() to get the value or panic with a custom error message
    let bad_result = divide_result(10.0, 0.0).unwrap_or_else(|e| {
        println!("Error: {}", e);
        0.0
    });

    // Using the ? operator to propagate errors
    fn divide_with_question_mark(a: f64, b: f64) -> Result<f64, String> {
        let result = divide_result(a, b)?; // This will return the error if divide_result returns an Err variant, or it will assign the successful value to result if divide_result returns an Ok variant.
        Ok(result) // Return the successful value wrapped in an Ok variant
    }

    match divide_with_question_mark(10.0, 0.0) {
        Ok(value) => println!("The result of division is: {}", value), // Handle the Ok variant and print the successful value
        Err(e) => println!("Error: {}", e), // Handle the Err variant and print the error message
    }
}
