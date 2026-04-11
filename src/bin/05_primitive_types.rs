fn main() {
    // Rust has several primitive types, including integers, floating-point numbers, booleans, and characters.
    // These types are built into the language and are not defined by the user.
    // They are used to represent simple values and are stored directly in memory.

    // Integer types include i8, i16, i32, i64, i128, isize (signed integers) and u8, u16, u32, u64, u128, usize (unsigned integers).
    // The number after the type indicates the number of bits that the type uses to store the value.
    // For example, i32 is a signed integer that uses 32 bits to store its value.

    let x: i32 = 5; // x is an integer variable of type i32
    println!("x = {}, ", x); // x = 5

    let x: i64 = 5; // x is an integer variable of type i64
    println!("x = {}, ", x); // x = 5

    // isize is a signed integer that uses the same number of bits as the platform's pointer type (either 32 or 64 bits). 
    // The choice between these types depends on the range of values you need to represent and the performance requirements of your program.
    
    let x: isize = 5; // x is an integer variable of type isize
    println!("x = {}, ", x); // x = 5

   
    

    // Floating-point types include f32 and f64. The number after the type indicates the number of bits that the type uses to store the value.
    // For example, f64 is a floating-point number that uses 64 bits to store its value.
   
    // f32 has a precision of about 6 decimal places and can represent values in the range of approximately 1.4E-45 to 3.4E38, while f64 has a precision of about 15 decimal places and can represent values in the range of approximately 5.0E-324 to 1.7E308.
    // In general, you should use f64 for most floating-point calculations unless you have a specific reason to use f32, such as when you need to save memory or when you are working with a large number of floating-point values and the precision of f32 is sufficient for your needs.

   
    let y:f32 = 3.14; // y is a floating-point variable of type f32
    println!("y = {}, ", y); // y = 3.14

    let y: f64 = 3.14; // y is a floating-point variable of type f64
    println!("y = {}, ", y); // y = 3.14

   
    // Boolean type
    let z: bool = true; // z is a boolean variable of type bool
    println!("z = {}, ", z); // z = true

    // Character type
    let c: char = 'A'; // c is a character variable of type char
    println!("c = {}, ", c); // c = A

    println!(); // Print a newline at the end

}
