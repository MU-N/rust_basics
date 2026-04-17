// Vectors are resizable arrays. They are provided by the standard library and are implemented as a struct called Vec<T>.
// - Vec<T> is a growable array type that can hold elements of any type T.
// - Vectors are stored on the heap, which allows them to grow and shrink dynamically as elements are added or removed.
// - Vectors only store values of the same type, eg. Vec<i32>.
// Created using 'Vec::new()' or 'vec![1, 2, 3]'.
// Add elements using 'push()' method.
// Access elements using indexing or 'get()' method.
// Remove elements using 'pop()' method.
// Accessing with '[]' will panic if the index is out of bounds, while 'get()' returns an Option<T> that can be handled safely.
// Iterate immutable with 'iter()' or ' for i in &vec'.
// mutable with 'iter_mut()' or ' for i in &mut vec'.

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(4); // Add an element to the end of the vector
    vec.push(5);
    vec.push(6);
    vec.push(1);
    println!("Vector: {:?}", vec); // Print the vector

    vec.sort(); // Sort the vector in ascending order
    println!("Sorted Vector: {:?}", vec); // Print the vector
    println!("First element: {}", vec[0]); // Access the first element using indexing
    
    
    match vec.get(10) { // Attempt to access an out-of-bounds index using get()
        Some(value) => println!("Element at index 10: {}", value), // Handle the case where the element is present
        None => println!("No element at index 10"), // Handle the case where the element is absent
    }

    println!("Popped element: {:?}", vec.pop().unwrap()); // Remove and print the last element of the vector


    let first_element: &mut i32 = vec.get_mut(0).unwrap(); 
    *first_element = 10; // Modify the first element through a mutable reference
    println!("Modified first element: {}", first_element);
    println!("Modified Vector: {:?}", vec); // Print the modified vector

    if vec.contains(&5) { // Check if the vector contains the value 5
        println!("Vector contains 5"); // Print a message if the value is found
    } else {
        println!("Vector does not contain 5"); // Print a message if the value is not found
    }

    if vec.get(100) == None
    {
        println!("No element at index 100");
    }


    for i in  &mut vec { // Iterate over the vector with mutable references
        *i += 1; // Increment each element by 1
    }
    println!("Modified Vector after iteration: {:?}", vec); // Print the modified vector
    
}
