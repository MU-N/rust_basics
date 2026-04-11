fn main()
{
    // Arrays are a fixed-size collection of elements of the same type. They are stored on the stack and have a fixed length that is determined at compile time.
    // The syntax for declaring an array is [type; length], where type is the type of the elements in the array and length is the number of elements in the array.  

    let a: [i32;5] = [1, 2, 3, 4, 5]; // a is an array of 5 integers
    println!("a = {:?}", a); // a = [1, 2, 3, 4, 5]

    // We can also initialize an array with the same value for all elements using the syntax [value; length].
    let b: [i32; 5] = [10; 5]; // b is an array of 5 integers, all initialized to 10
    println!("b = {:?}", b); // b = [10, 10, 10, 10, 10]

    // We can access individual elements of an array using indexing. The index starts at 0, so the first element of the array is accessed with index 0, the second element with index 1, and so on.
    let first_element = a[0]; // first_element is 1
    println!("The first element of a is: {}", first_element); // The first element of a is: 1
    let second_element = a[1]; // second_element is 2
    println!("The second element of a is: {}", second_element); // The second element of
    // We can also use a loop to iterate over the elements of an array. For example, we can use a for loop to print each element of the array.
   for i in 0 ..a.len() {
        println!("Element {} of a is: {}", i, a[i]);
    }


}