# 📦 Rust Vectors: Dynamic Arrays

In Rust, a **Vector** (`Vec<T>`) is a **resizable array** that stores values of the same type. Unlike fixed-size arrays, vectors can grow and shrink dynamically during runtime.

---

## 🚀 Why Use Vectors?

Vectors are useful when:

* You don't know the number of elements ahead of time.
* You want to modify (add/remove) elements dynamically.
* You need safe, efficient memory handling with ownership and borrowing rules.

---

## 🧪 Basic Usage

```rust
fn main() {
    let mut numbers = Vec::new(); // create an empty vector

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("{:?}", numbers); // [10, 20, 30]
}
```

* `Vec::new()` creates an empty vector.
* `.push()` adds elements to the end.
* `println!("{:?}", ...)` prints the entire vector.

---

## 🧱 Initializing with Values

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    println!("{:?}", numbers);
}
```

* `vec![...]` is a macro to create a vector with predefined values.

---

## 📚 Accessing Elements

```rust
fn main() {
    let numbers = vec![100, 200, 300];

    println!("{}", numbers[0]);         // prints 100
    println!("{}", numbers.get(1).unwrap()); // prints 200
}
```

* Use `[]` for direct indexing.
* Use `.get(index)` for **safe access** (returns `Option<T>`).
* `.unwrap()` is used here for simplicity; handle errors properly in production code.

---

## ⚠️ Safe Access with `.get()`

```rust
fn main() {
    let data = vec![1, 2, 3];

    match data.get(10) {
        Some(value) => println!("Found: {}", value),
        None => println!("Index out of bounds!"),
    }
}
```

---

## 🔁 Iterating Over Vectors

```rust
fn main() {
    let items = vec!["apple", "banana", "cherry"];

    for item in &items {
        println!("{}", item);
    }
}
```

* Use `&items` to iterate by reference (avoids moving values).
* You can also use `.iter()` for the same effect.

---

## ✍️ Modifying Values in a Vector

```rust
fn main() {
    let mut scores = vec![10, 20, 30];

    for score in &mut scores {
        *score += 5;
    }

    println!("{:?}", scores); // [15, 25, 35]
}
```

* Use `&mut vector` for mutable iteration.
* You must dereference with `*` to modify the value.

---

## 🧼 Removing Elements

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4];

    stack.pop(); // removes the last element
    println!("{:?}", stack); // [1, 2, 3]
}
```

* `.pop()` removes and returns the **last element**.

---

## 🛡️ Ownership and Borrowing in Vectors

```rust
fn main() {
    let mut names = vec!["Alice".to_string(), "Bob".to_string()];

    let first = &names[0];
    // names.push("Charlie".to_string()); // ❌ Error: cannot borrow `names` as mutable because it's also borrowed as immutable

    println!("First name: {}", first);
}
```

Rust **prevents mutable and immutable borrows from coexisting**, ensuring memory safety.

---

## 🔄 Common Vector Methods

| Method          | Description                       |
| --------------- | --------------------------------- |
| `.push(val)`    | Adds a value to the end           |
| `.pop()`        | Removes and returns the last item |
| `.get(i)`       | Safely accesses item at index `i` |
| `.remove(i)`    | Removes item at index `i`         |
| `.insert(i, v)` | Inserts `v` at index `i`          |
| `.len()`        | Returns the number of elements    |
| `.clear()`      | Removes all elements              |
| `.contains(&x)` | Checks if a value exists          |
| `.iter()`       | Returns an iterator               |

---

## ✅ Summary

| Feature          | Description                                   |
| ---------------- | --------------------------------------------- |
| `Vec<T>`         | Growable array for storing same-type values   |
| Dynamic resizing | Add/remove elements with `.push()` / `.pop()` |
| Safe access      | Use `.get()` to prevent panics                |
| Ownership safe   | Rust ensures borrowing rules are followed     |

---

## 🧠 Final Thoughts

Vectors are **powerful**, **flexible**, and **safe**. They're one of the most used collection types in Rust. Once you understand them, you'll find handling data collections in Rust becomes much easier!

---
