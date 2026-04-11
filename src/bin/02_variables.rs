fn main()
{
    let x :i32 = 10;
    println!("x = {}", x);
    
    // x = 20; // This will cause a compile-time error because `x` is immutable by default.
    let x = 20; // This is called "shadowing". It allows us to reuse the variable name `x` with a new value.
    println!("x = {}", x);

    // Shadowing can also be used to change the type of a variable.
    let spaces = "   "; // `spaces` is a string slice.
    println!("spaces = '{}'", spaces);
    let spaces = spaces.len(); // Now `spaces` is an integer representing the length of the string.
    println!("spaces = {}", spaces);

    // We can use mutable variables if we want to change the value without shadowing.
    let mut y = 5; // `y` is mutable, so we can change its value.
    println!("y = {}", y);  

    y = 10; // This is allowed because `y` is mutable.
    println!("y = {}", y);


}