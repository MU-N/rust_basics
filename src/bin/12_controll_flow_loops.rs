// Loops
// Rust provides several types of loops for repeating code:
// 1. loop: This is an infinite loop that will continue to execute until it is explicitly broken out of using the break keyword.
// 2. while: This loop will continue to execute as long as a specified condition is true. The condition is evaluated before each iteration of the loop, so if the condition is false at the beginning, the loop will not execute at all.
// 3. for: This loop is used to iterate over a range of values or over a collection of items. The syntax for a for loop is for variable in collection { }.

fn main() {
    // loop example
    /*let mut count: i32 = 0; // count is a mutable variable initialized to 0

    loop {
        println!("Count: {}", count); // Print the current value of count
        count += 1; // Increment count by 1

        if count >= 50 { // If count is greater than or equal to 5, break out of the loop
            break;
        }
    }*/

    /*
    // while example
    let mut number: i32 = 0; // number is a mutable variable initialized to
    while number < 5 { // While number is less than 5, execute the loop
        println!("Number: {}", number); // Print the current value of number
        number += 1; // Increment number by 1
    }*/

    // for example
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // numbers is an array of 5 integers

    // We can use a for loop to iterate over the elements of the array. The syntax for a for loop is for variable in collection { }, where variable is a new variable that takes on the value of each element in the collection, and collection is the array or other iterable collection that we want to iterate over.
    for i in 0..numbers.len() {
        // Iterate over the indices of the numbers array
        println!("Number: {}", numbers[i]); // Print the current number at index i
    }

    println!("-------------------------------");

    //` We can also use a for loop to iterate over the elements of the array directly, without needing to use indexing. The syntax for iterating over the elements of an array is for variable in array.iter() { }, where variable is a new variable that takes on the value of each element in the array, and array.iter() returns an iterator over the elements of the array.`
    for number in numbers.iter() {
        // Iterate over the elements of the numbers array directly
        println!("Number: {}", number); // Print the current number
    }

    println!("-------------------------------");

    // We can also use a for loop to iterate over the indices of the array in reverse order. The syntax for iterating over the indices of an array in reverse order is for variable in (0..array.len()).rev() { }, where variable is a new variable that takes on the value of each index in the reversed range, and (0..array.len()).rev() creates a reversed range of indices from array.len()-1 (inclusive) to 0 (inclusive).
    for i in numbers.iter().rev() {
        // Iterate over the indices of the numbers array in reverse order
        println!("Number: {}", i); // Print the current number at index i
    }

    println!("-------------------------------");

    // We can also use a for loop to iterate over a range of numbers. The syntax for iterating over a range of numbers is for variable in start..end { }, where variable is a new variable that takes on the value of each number in the range, and start..end creates a range of numbers from start (inclusive) to end (exclusive).
    for i in 0..5 {
        // Iterate over the range of numbers from 0 to 4
        println!("i: {}", i); // Print the current value of i
    }

    println!("-------------------------------");
    // We can also use the rev method to reverse the order of the range of numbers. The syntax for reversing a range of numbers is for variable in (start..end).rev() { }, where variable is a new variable that takes on the value of each number in the reversed range, and (start..end).rev() creates a reversed range of numbers from end-1 (inclusive) to start (inclusive).
    for i in (0..5).rev() {
        // Iterate over the range of numbers from 0 to 4
        println!("i: {}", i); // Print the current value of i
    }
}
