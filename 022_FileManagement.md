# 📂 File Management in Rust: Create, Read, Update, Delete (CRUD)

Rust provides a powerful and safe way to work with the file system using the `std::fs` and `std::io` modules. In this guide, you’ll learn how to **Create**, **Read**, **Update**, and **Delete** files (CRUD operations) in Rust.

---

## 📄 1. Creating and Writing to a File

You can use `File::create()` to create a new file. Use `write_all()` to write content.

```rust
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("data.txt").expect("Could not create file");

    file.write_all(b"Hello, Rust!\n").expect("Write failed");
    println!("File created and written successfully!");
}
````

* If the file already exists, it will be **overwritten**.
* The `b""` prefix makes the string a byte literal.

---

## 📖 2. Reading from a File

Use `fs::read_to_string()` to read an entire file's contents.

```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("data.txt").expect("Could not read file");

    println!("File content:\n{}", content);
}
```

* This reads the whole file into a `String`.
* You can also use `BufReader` for large files.

---

## ✍️ 3. Updating a File (Append Mode)

Use `OpenOptions` to open a file in **append** mode.

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("Cannot open file");

    file.write_all(b"Appended line\n").expect("Write failed");

    println!("Appended to file successfully!");
}
```

* `append(true)` tells Rust to add content to the end of the file.
* Use `.read(true)` or `.write(true)` for other modes.

---

## ❌ 4. Deleting a File

Use `fs::remove_file()` to delete a file.

```rust
use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Could not delete file");

    println!("File deleted successfully!");
}
```

* Always check that the file exists before deleting it if you're not sure.

---

## 🧪 Full CRUD Example

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};

fn main() {
    // CREATE
    let mut file = File::create("example.txt").expect("Create failed");
    file.write_all(b"Line 1\n").expect("Write failed");

    // READ
    let content = fs::read_to_string("example.txt").expect("Read failed");
    println!("Initial content:\n{}", content);

    // UPDATE (Append)
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")
        .expect("Open failed");
    file.write_all(b"Line 2\n").expect("Append failed");

    // READ AGAIN
    let content = fs::read_to_string("example.txt").expect("Read failed");
    println!("After appending:\n{}", content);

    // DELETE
    fs::remove_file("example.txt").expect("Delete failed");
    println!("File deleted.");
}
```

---

## 🔐 Error Handling Tips

Always handle file operations with care. For real-world applications, use pattern matching:

```rust
match fs::read_to_string("data.txt") {
    Ok(content) => println!("Content: {}", content),
    Err(e) => eprintln!("Error reading file: {}", e),
}
```

---

## 📚 Useful Methods & Traits

| Function / Type        | Description                       |
| ---------------------- | --------------------------------- |
| `File::create()`       | Create a new file                 |
| `OpenOptions::new()`   | Open with options (append, read…) |
| `fs::read_to_string()` | Read the file into a string       |
| `fs::remove_file()`    | Delete a file                     |
| `write_all()`          | Write bytes to a file             |
| `std::io::BufReader`   | Efficient buffered reading        |

---

## ✅ Summary

| Operation | Method                        |
| --------- | ----------------------------- |
| Create    | `File::create()`              |
| Read      | `fs::read_to_string()`        |
| Update    | `OpenOptions` with `append()` |
| Delete    | `fs::remove_file()`           |

Rust’s file APIs are powerful, type-safe, and robust—ideal for building reliable CLI tools, apps, and backends.

---

## 📎 Further Reading

* [Rust Book - File I/O](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
* [std::fs documentation](https://doc.rust-lang.org/std/fs/index.html)
* [std::io documentation](https://doc.rust-lang.org/std/io/index.html)

---
