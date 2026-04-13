// Match is a powerful control flow operator in Rust that allows you to compare a value against a series of patterns and execute code based on which pattern matches.
// It is similar to a switch statement in other programming languages, but it is more powerful and flexible because it can match complex patterns and destructure values.
// The syntax for a match expression is as follows:
// match value { pattern1 => expression1, pattern2 => expression2, ... }

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    name: String,
    health: i32,
    position: (f64, f64, f64, f64),
}

impl Player {
    fn move_player(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.1 += 1.0, // Move up by increasing the y-coordinate
            Direction::Down => self.position.1 -= 1.0, // Move down by decreasing the y-coordinate
            Direction::Left => self.position.0 -= 1.0, // Move left by decreasing the x-coordinate
            Direction::Right => self.position.0 += 1.0, // Move right by increasing the x-coordinate
        }
    }
}

fn main() {
    let direction: Direction = Direction::Up;
    let mut player = Player {
        name: String::from("Alice"),
        health: 100,
        position: (0.0, 0.0, 0.0, 1.0),
    };
    match direction {
        Direction::Up => player.move_player(Direction::Up), // Move the player up
        Direction::Down => player.move_player(Direction::Down), // Move the player down
        Direction::Left => player.move_player(Direction::Left), // Move the player left
        Direction::Right => player.move_player(Direction::Right), // Move the player right
    }


    // We can also use a match expression to match against literal values, such as integers or strings. 
    // For example, we can match against the value of an integer variable like this:
    let x:i32 = 5;
    match x {
        1 => println!("x is 1"), // Match the value of x against the pattern 1
        2 => println!("x is 2"), // Match the value of x against the pattern 2
        3 => println!("x is 3"), // Match the value of x against the pattern 3
        other => println!("x is something else"), // Match any other value of x using the wildcard pattern _
    }

    // We can also use a match expression to match against multiple patterns using the | operator. 
    // For example, we can match against the values 1 or 2 like this:
    // We can also use a match expression to match against a range of values using the ..= operator. 
    // For example, we can match against the values between 3 and 5 (inclusive) like this:
    // We can also combine multiple patterns and ranges in a single match expression.
    // For example, we can match against the values 1 or 2, or the values between 3 and 5 like this:

    match x {
        1 | 2 => println!("x is 1 or 2"), // Match the value of x against the patterns 1 or 2
        3..=5 => println!("x is between 3 and 5"), // Match the value of x against the range pattern 3..=5
        10|12 | 5..=10 => println!("x is 1, 2, or between 3 and 5"), // Match the value of x against the patterns 1 or 2, or the range pattern 3..=5
        _ => println!("x is something else"), // Match any other value of x using the wildcard pattern _
        
    }


}
