// Constants are always immutable values that are bound to a name and can be used throughout the program. They must have a type annotation and can be declared in any scope, including the global scope.
// we must to identify the type of the constant, and we can use underscores to make large numbers more readable.
const MAX_POINTS: u32 = 100_000;

fn main() {
    // Constants are always immutable and must have a type annotation. They can be declared in any scope, including the global scope.
    println!("MAX_POINTS = {}", MAX_POINTS);
}
