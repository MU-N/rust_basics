
// Structs are a way to create custom data types in Rust. They allow us to group together related data and define our own types that can have their own fields and methods. Structs are similar to classes in other programming languages, but they do not have inheritance or polymorphism. Instead, they are used to create simple data structures that can be used to represent complex data in a more organized way.

struct Vector4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

struct Player{
    name: String,
    health: i32,
    position: Vector4,
    color: Color,
}

// Tuples struct are a type of struct that is defined using a tuple syntax. 
// They are similar to regular structs, but they do not have named fields. 
// Instead, the fields of a tuple struct are accessed using their index, starting from 0. 
// Tuple structs are useful when you want to create a simple data structure that does not require named fields, and they can be used to create more concise code in certain situations.
struct Color(u8, u8, u8); // A tuple struct that represents a color with three components: red, green, and blue


fn main()
{
    let mut player1 = Player {
        name: String::from("Alice"),
        health: 100,
        position: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        color: Color(255, 0, 0), // Red color
    };

    println!("Player Name: {}", player1.name); // Player Name: Alice
    println!("Player Health: {}", player1.health); // Player Health: 100
    println!("Player Position: ({}, {}, {}, {})", player1.position.x, player1.position.y, player1.position.z, player1.position.w); // Player Position: (0.0, 0.0, 0.0, 1.0)
    println!("Player Color: ({}, {}, {})", player1.color.0, player1.color.1, player1.color.2); // Player Color: (255, 0, 0)


    // We can also modify the fields of a struct after it has been created. To modify a field, we need to make the struct mutable by using the mut keyword when declaring the variable. Then we can access the fields of the struct using dot notation and assign new values to them.
    player1.health -= 10; // Decrease player1's health by 10
    player1.position.x += 1.0; // Move player1's position along the
    player1.position.y += 1.0; // Move player1's position along the y-axis
    player1.position.z += 1.0; // Move player1's position along the
    player1.position.w += 0.0; // Move player1's position along the w-axis (no change)
   
    println!("Player Name: {}", player1.name); // Player Name: Alice
    println!("Player Health: {}", player1.health); // Player Health: 90 
    println!("Player Position: ({}, {}, {}, {})", player1.position.x, player1.position.y, player1.position.z, player1.position.w); // Player Position: (1.0, 1.0, 1.0, 1.0)
    println!("Player Color: ({}, {}, {})", player1.color.0, player1.color.1, player1.color.2); // Player Color: (255, 0, 0)



   // We can also create new instances of a struct using the values of an existing instance. 
    // This is done using the struct update syntax, which allows us to specify the fields that we want to change and use the remaining fields from the existing instance.
    let player2 = Player {
        name: String::from("Bob"),
        health: 80,
      //  ..player1 // Use the remaining fields from player1
        position: Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }, // Override the position field with a new value
        color: Color(255, 0, 0), // Red color
    };

    println!("Player Name: {}", player2.name); // Player Name: Bob
    println!("Player Health: {}", player2.health); // Player Health: 80
    println!("Player Position: ({}, {}, {}, {})", player2.position.x, player2.position.y, player2.position.z, player2.position.w); // Player Position: (1.0, 1.0, 1.0, 1.0)
    println!("Player Color: ({}, {}, {})", player2.color.0, player2.color.1, player2.color.2); // Player Color: (255, 0, 0)



    // We can also define methods for a struct, which are functions that are associated with the struct and can be called on instances of the struct. 
    // Methods are defined using the impl keyword, followed by the name of the struct. Inside the impl block, we can define methods that take self as a parameter, which refers to the instance of the struct that the method is being called on.
    impl Player {
        fn take_damage(&mut self, damage: i32) {
            self.health -= damage; // Decrease the player's health by the specified damage
        }

        fn move_position(&mut self, dx: f64, dy: f64, dz: f64) {
            self.position.x += dx; // Move the player's position along the x-axis
            self.position.y += dy; // Move the player's position along the y-axis
            self.position.z += dz; // Move the player's position along the z-axis
        }
    }

    player1.take_damage(20); // player1 takes 20 damage
    player1.move_position(2.0, 2.0, 2.0); // player1 moves to a new position
    println!("Player Name: {}", player1.name); // Player Name: Alice
    println!("Player Health: {}", player1.health); // Player Health: 70
   // if we  "..player1" in player2, then player2's position would be the same as player1's position, which is (1.0, 1.0, 1.0, 1.0) after the move_position method is called on player1. 
   // and we will not be abel to print the player1 position after the move_position method is called on player1, because player2's position would be the same as player1's position, and we would not be able to distinguish between the two players' positions.
    println!("Player Position: ({}, {}, {}, {})", player1.position.x, player1.position.y, player1.position.z, player1.position.w); // Player Position: (3.0, 3.0, 3.0, 1.0)
    println!("Player Color: ({}, {}, {})", player1.color.0, player1.color.1, player1.color.2); // Player Color: (255, 0, 0)

}