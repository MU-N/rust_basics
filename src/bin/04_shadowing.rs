
// shadowing is a powerful feature in Rust that allows you to declare a new variable with the same name as a previous variable. 
// new variable shadows the previous variable, meaning that the previous variable is no longer accessible. 
// s can be useful for reusing variable names and for changing the type of a variable without needing to create a new name.
fn main()
{

    let x: i32 = 5;
    let x: i32 = x + 1; // shadowing the previous variable x
    let x: i32 = x * 2; // shadowing the previous variable x again

    println!("The value of x is: {}", x); // The value of x is: 12

    // Shadowing can also be used to create a new variable with the same name in a different scope.
    {
        let x: &str = "hello"; // shadowing the previous variable x in a new scope
        println!("The value of x in the inner scope is: {}", x); // The value of x in the inner scope is: hello
    }


    // Shadowing can also be used to change the type of a variable
    let spaces: &str = "   "; // spaces is a string slice
    let spaces: usize = spaces.len(); // spaces is now an integer

    println!("The number of spaces is: {}", spaces); // The number of spaces is: 3

}