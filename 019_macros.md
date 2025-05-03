# 🧰 Macros in Rust

Rust macros are powerful **compile-time code generators** that enable **code reuse**, **abstraction**, and **metaprogramming**.

---

## 🧠 What is a Macro?

A **macro** in Rust lets you write code that writes other code. Macros are expanded at compile time and differ from functions because they can:

- Accept variable numbers of arguments
- Generate repetitive or pattern-based code
- Implement DSL-like syntax

---

## ✅ Common Built-in Macros

| Macro         | Description                                  |
|---------------|----------------------------------------------|
| `println!`    | Prints to the console with formatting         |
| `format!`     | Returns a formatted `String`                  |
| `vec!`        | Creates a `Vec`                              |
| `dbg!`        | Debug prints a value                         |
| `assert!`     | Checks a condition and panics if false       |
| `include_str!`| Includes file contents as a string at compile time |

Example:

```rust
fn main() {
    let name = "Alice";
    println!("Hello, {}!", name); // Macro with variable input
}
````

---

## 🧱 Declarative Macros: `macro_rules!`

This is the traditional way to define macros in Rust.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

fn main() {
    say_hello!(); // Expands into println!
}
```

### 🧩 Macro with Parameters

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}
```

* `stringify!` turns code tokens into a string.
* `$func_name:ident` captures an identifier (like a function name).

---

## 🔁 Repetition with Macros

```rust
macro_rules! print_numbers {
    ( $( $x:expr ),* ) => {
        $(
            println!("Number: {}", $x);
        )*
    };
}

fn main() {
    print_numbers!(10, 20, 30, 40);
}
```

* `$( $x:expr ),*` matches zero or more comma-separated expressions.
* `$()*` applies a block for each match.

---

## 🧪 Macro to Implement Traits

```rust
macro_rules! make_sum {
    ($name:ident) => {
        fn $name(a: i32, b: i32) -> i32 {
            a + b
        }
    };
}

make_sum!(add_two);

fn main() {
    println!("3 + 5 = {}", add_two(3, 5));
}
```

---

## 🧠 When to Use Macros

| Use Case                         | Use                |
| -------------------------------- | ------------------ |
| Code generation                  | ✅                  |
| DSL or custom syntax             | ✅                  |
| Repetitive trait implementations | ✅                  |
| Compile-time operations          | ✅                  |
| General-purpose logic            | ❌ Prefer functions |

---

## ⚠️ Macro Tips

* Debug using `cargo expand` to see generated code.
* Prefer functions for simple tasks.
* Macros can be hard to read and debug—use responsibly.

---

## 🧪 Procedural Macros (Advanced)

Rust also supports:

* **Custom #\[derive] macros**
* **Function-like procedural macros**
* **Attribute macros**

These are declared in separate crates with the `proc-macro` attribute and offer even more control.

Example of using `#[derive(Debug)]` is a built-in procedural macro:

```rust
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}
```

---

## 📚 Further Reading

* 📘 [Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
* 🔎 [Macros by Example - Reference](https://doc.rust-lang.org/reference/macros-by-example.html)
* 🧪 [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)

---

## ✅ Summary

| Concept            | Description                         |
| ------------------ | ----------------------------------- |
| `macro_rules!`     | Defines a pattern-based macro       |
| `$()`              | Match and repeat syntax             |
| `proc-macro`       | Advanced macros, like `#[derive()]` |
| `stringify!`       | Converts code to string             |
| `vec!`, `println!` | Built-in declarative macros         |

---

🚀 Mastering macros allows you to build **powerful**, **reusable**, and **expressive** code in Rust—just don’t overdo it! For simple logic, functions are your friends. ✨

---
