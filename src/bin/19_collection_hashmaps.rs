
// HashMap<K, V> stores key-value pairs. 
// - HashMap<K, V> is a collection that maps keys of type K to values of type V.
// - HashMaps are stored on the heap, allowing them to grow and shrink dynamically as key-value pairs are added or removed.
// Created using 'HashMap::new()' or 'collect()' method on an iterator of key-value pairs.
// Add key-value pairs using 'insert()' method.
// Access values using 'get()' method, which returns an Option<&V> that can be handled safely.
// Remove key-value pairs using 'remove()' method.
// HashMaps do not maintain the order of key-value pairs, and keys must be unique.

use std::collections::HashMap; // Import the HashMap type from the standard library



fn main()
{

    let mut map = HashMap::new(); // Create a new HashMap
    map.insert("key1", 10); // Insert a key-value pair into the HashMap
    map.insert("key2", 20);
    println!("HashMap: {:?}", map); // Print the HashMap


    match map.get("key1") { // Attempt to access a value using a key
        Some(value) => println!("Value for 'key1': {}", value), // Handle the case where the key is present
        None => println!("No value for 'key1'"), // Handle the case where the key is absent
    }

    if map.get("key3") == None { // Check if a key is present in the HashMap
        println!("No value for 'key3'"); // Print a message if the key is not found
    }

    println!("Removed value for 'key2': {:?}", map.remove("key2")); // Remove a key-value pair and print the removed value
    println!("HashMap after removal: {:?}", map); // Print the HashMap after removal


    for (key, value) in &map { // Iterate over the key-value pairs in the HashMap
        println!("Key: {}, Value: {}", key, value); // Print each key and its corresponding value
    }

    map.insert("key1", 30); // Update the value for an existing key
    println!("Updated HashMap: {:?}", map); // Print the updated HashMap

    map.insert("key3", 40); // Insert a new key-value pair into the HashMap
    println!("HashMap after adding 'key3': {:?}", map); // Print the HashMap after adding a new key-value pair

    map.insert("key4", 50); // Insert another key-value pair into the HashMap
    println!("HashMap after adding 'key4': {:?}", map); // Print the HashMap

    map.remove("key1"); // Remove a key-value pair from the HashMap
    println!("HashMap after removing 'key1': {:?}", map); // Print the HashMap

}