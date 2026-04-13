
// Control Flow: if statements
// Syntax: if condition { } else if condition { } else { }

fn main()
{
    let number: i32 = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is equal to 10");
    } else {
        println!("The number is greater than 10");
    }

    // We can also use if statements in a more concise way using the ternary operator syntax, which is a shorthand for an if-else statement. The syntax for the ternary operator is condition ? expression_if_true : expression_if_false.
    let is_even: bool = if number % 2 == 0 { true } else { false };
    println!("Is the number even? {}", is_even); // Is the number even? false

    // We can also use if statements in a more concise way using the ternary operator syntax, which is a shorthand for an if-else statement. The syntax for the ternary operator is condition ? expression_if_true : expression_if_false.
    let is_odd: bool = if number % 2 != 0 { true } else { false };
    println!("Is the number odd? {}", is_odd); // Is the number odd? true

    
}