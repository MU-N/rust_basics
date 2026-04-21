

#[derive(Debug, Clone)]

struct Product
{
    id: u32,
    name: String,
    price: f64,
    catogory: Catogory,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Catogory
{
    Food,
    Electronics,
    Clothing,
    Books,
}



fn main()
{

    let mut inventory: Vec<Product> = Vec::new(); // Create a new vector to store products

    add_product(&mut inventory, 1, "Apple".to_string(), 0.99, Catogory::Food); // Add a product to the inventory
    add_product(&mut inventory, 2, "Laptop".to_string(), 999.99, Catogory::Electronics);
    add_product(&mut inventory, 3, "T-Shirt".to_string(), 19.99, Catogory::Clothing);   

    println!("Inventory: {:?}", inventory); // Print the inventory
    match find_product(inventory.clone(), 2) { // Attempt to find a product by its ID
        Some(product) => println!("Found product: {:?}", product), // Handle the case where the product is found
        None => println!("Product not found"), // Handle the case where the product is not found
    }

}  

fn add_product(mut inventory: &mut Vec<Product>, id: u32, name: String, price: f64, catogory: Catogory)
{
    let product = Product { id, name, price, catogory }; // Create a new product
    inventory.push(product); // Add the product to the inventory
}

fn find_product( inventory :Vec<Product>, id: u32) -> Option<Product>
{
    for product in inventory { // Iterate through the inventory to find a product by its ID
        if product.id == id {
            return Some(product); // Return the product if found
        }
    }
    None // Return None if the product is not found
}

fn remove_product(mut inventory: &mut Vec<Product>, id: u32) -> Option<Product>
{
    if let Some(pos) = inventory.iter().position(|product| product.id == id) { // Find the position of the product to remove
        return Some(inventory.remove(pos)); // Remove and return the product if found
    }
    None // Return None if the product is not found
}

