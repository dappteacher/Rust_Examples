use std::io;

struct Product {
    name: String,
    weight: f32,
    unit: String,
}

fn main() {
    let mut products: Vec<Product> = Vec::new();

    add_product(&mut products);

    println!("\nCurrent Products:");
    for product in &products {
        println!("{}, {} {}", product.name, product.weight, product.unit);
    }

    println!("\nEnter product name to search:");
    let mut search_name = String::new();
    io::stdin()
        .read_line(&mut search_name)
        .expect("Failed to read input.");
    let search_name = search_name.trim();

    match get_product_info(&products, search_name) {
        Some((weight, unit)) => println!("Found: {} {}", weight, unit),
        None => println!("Product not found."),
    }
}

/// Adds a single product to the list by prompting user input
fn add_product(products: &mut Vec<Product>) {
    let name = read_input("Please enter the name of the product:");
    let weight: f32 = read_input("Please enter the weight of the product:")
        .parse()
        .expect("Invalid number for weight.");
    let unit = read_input("Please enter the unit of the product:");

    let product = Product { name, weight, unit };
    products.push(product);
}

/// Utility function to read and trim input from the user
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input.");
    buffer.trim().to_string()
}

/// Searches for a product by name and returns its weight and unit if found
fn get_product_info(products: &[Product], search_name: &str) -> Option<(f32, String)> {
    for product in products {
        if product.name.eq_ignore_ascii_case(search_name) {
            return Some((product.weight, product.unit.clone()));
        }
    }
    None
}
