fn main()
{
    // Tuples are a fixed-size collection of values of different types. They are stored on the stack and have a fixed length that is determined at compile time.
    // The syntax for declaring a tuple is (type1, type2, type3, ...), where type1, type2, type3, etc. are the types of the values in the tuple.
    let t :(i32, bool, char, f64) = (42, true, 'A', 3.14); // t is a tuple of 4 values of different types
    println!("t = {:?}", t); // t = (42, true, 'A', 3.14)

    // We can access individual values of a tuple using indexing. The index starts at 0, so the first value of the tuple is accessed with index 0, the second value with index 1, and so on.
    let first_value = t.0; // first_value is 42
    println!("The first value of t is: {}", first_value); // The first value of

    // We can also use a pattern matching syntax to destructure a tuple into individual variables. This allows us to access the values of the tuple without needing to use indexing.
    let (x, y, z, w) = t; // x is 42, y is true, z is 'A', w is 3.14
    println!("x = {}, y = {}, z = {}, w = {}", x, y, z, w); // x = 42, y = true, z = 'A', w = 3.14
}