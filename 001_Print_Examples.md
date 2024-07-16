// Rust Print Example Series

### Series 1: Basic Printing
```rust
fn main() {
    // Print without newline
    print!("I am energetic!");
    print!("Number is: {}", 16);
    println!();

    // Print with newline
    println!("I am energetic!");
    println!("Number is: {}", 16);

    // Formatted string
    let formatted_string = format!("Hello, {}!", "Rust");
    println!("{}", formatted_string);

    // Print to stderr without newline
    eprint!("An error occurred!");
    println!();

    // Print to stderr with newline
    eprintln!("An error occurred!");

    // Another formatted string
    let formatted_string = format!("Hello, {}!", "world");
    println!("{}", formatted_string);

    // Print variable values
    let x = 5;
    println!("The value of x is: {}", x);
    println!("The value of x is: {x}");

    // Print multiple variables
    let a = 16;
    let b = 25;
    println!("First is: {} and Second is: {}", a, b);
    println!("{} * {} = {} * {}", a, b, b, a);

    // Print arrays
    let my_array = [1, 2, 3];
    println!("The array is: {:?}", my_array);

    let new_array = [1, 2, 3];
    println!("The array is: {:#?}", new_array);

    // Print floating-point with precision
    let x = 3.141592;
    println!("Pi is roughly {:.2}", x);

    // Complex formatted print
    println!("{1:?} {0:?} is the {developer:?} name.", "King", "Alex", developer = "developer's");

    // Debug print
    let x = 42;
    dbg!(x);

    // Print environment variable
    println!("Current user: {}", std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()));

    // Conditional compilation
    #[cfg(debug_assertions)]
    println!("This is a debug build");

    #[cfg(not(debug_assertions))]
    println!("This is a release build");
}
```
### Series 2: Writing to stdout
```rust
use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    write!(stdout, "Hello, ").unwrap();
    writeln!(stdout, "world!").unwrap();
}
```
### Series 3: Writing to a buffer
```rust
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    write!(buffer, "Buffered ").unwrap();
    writeln!(buffer, "output!").unwrap();
    println!("{}", buffer);
}
```
### Series 4: Printing a single environment variable
```rust
use std::env;

fn main() {
    if let Ok(user) = env::var("USER") {
        println!("Current user: {}", user);
    } else {
        println!("USER environment variable is not set.");
    }
}
```
### Series 5: Printing multiple environment variables
```rust
use std::env;

fn main() {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let home = env::var("HOME").unwrap_or_else(|_| "not set".to_string());
    let shell = env::var("SHELL").unwrap_or_else(|_| "not set".to_string());

    println!("User: {}", user);
    println!("Home directory: {}", home);
    println!("Shell: {}", shell);
}
```
### Series 6: Function to print an environment variable
```rust
use std::env;

fn print_env_var(key: &str) {
    match env::var(key) {
        Ok(value) => println!("{}: {}", key, value),
        Err(_) => println!("{} environment variable is not set.", key),
    }
}

fn main() {
    print_env_var("USER");
    print_env_var("HOME");
    print_env_var("SHELL");
}
```
### Series 7: Iterating over all environment variables
```rust
use std::env;

fn main() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
```
### Series 8: Filtering environment variables
```rust
use std::env;

fn main() {
    let prefix = "RUST";
    for (key, value) in env::vars() {
        if key.starts_with(prefix) {
            println!("{}: {}", key, value);
        }
    }
}
```
### Series 9: Conditional behavior based on an environment variable
```rust
use std::env;

fn main() {
    if let Ok(debug) = env::var("DEBUG") {
        if debug == "true" {
            println!("Debug mode is enabled.");
        } else {
            println!("Debug mode is disabled.");
        }
    } else {
        println!("DEBUG environment variable is not set.");
    }
}
```
### Series 10: Setting and printing environment variables
```rust
use std::env;

fn main() {
    env::set_var("MY_VAR", "value");
    if let Ok(my_var) = env::var("MY_VAR") {
        println!("MY_VAR: {}", my_var);
    } else {
        println!("MY_VAR environment variable is not set.");
    }
}
```
