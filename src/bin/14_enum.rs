
// Enums in Rust are a powerful way to define a type that can be one of several different variants. 
// Each variant can have its own associated data, and you can use pattern matching to handle each variant in a clean and concise way. 
// Enums are often used to represent states, options, or different types of data in a program.

enum Gender {
    Male,
    Female,
}

// Enums can also have associated data, which allows you to store additional information for each variant. 
// For example, we can define an enum called Animal that has three variants: Dog, Cat, and Bird. 
// Each variant can have its own associated data, such as a name for the dog, a name and age for the cat, and a name and a boolean indicating if the bird can fly. 
enum Animal {
    Dog(String), // A variant that represents a dog with a name stored as a string
    Cat { name: String, age: u8 }, // A variant that represents a cat with a name and age stored as a struct
    Bird { name: String, can_fly: bool }, // A variant that represents a bird with a name and a boolean indicating if it can fly
}


fn main()
{
    let gender: Gender = Gender::Male; // gender is an instance
    match gender {
        Gender::Male => println!("The gender is Male"),
        Gender::Female => println!("The gender is Female"),
    }


     // We can also create instances of the Animal enum with associated data and use pattern matching to handle each variant and access the associated data.
    let my_pet = Animal::Cat { name: String::from("Whiskers"), age: 3 }; // my_pet is an instance of the Cat variant of the Animal enum with associated data
    match my_pet {  
        Animal::Dog(name) => println!("My pet is a dog named {}", name), // Handle the Dog variant and access the name
        Animal::Cat { name, age } => println!("My pet is a cat named {} who is {} years old", name, age), // Handle the Cat variant and access the name and age
        Animal::Bird { name, can_fly } => println!("My pet is a bird named {} who can fly: {}", name, can_fly), // Handle the Bird variant and access the name and can_fly boolean
    }

    // Option enum is a special enum in Rust that is used to represent a value that can either be present (Some) or absent (None).

    let some_number: Option<i32> = Some(5); // some_number is an instance of the Some variant of the Option enum with an associated value of 5
    let no_number: Option<i32> = None; // no_number is an instance of the None variant of the Option enum, indicating that there is no value present
    match no_number {
        Some(n) => println!("The number is: {}", n), // Handle the Some variant and access the value n
        None => println!("No number is present"), // Handle the None variant
    }

}