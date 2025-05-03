# 📬 Messaging Between Threads in Rust

In Rust, **threads** can **communicate with each other** using **channels**. This technique avoids shared-state concurrency and is often **safer and simpler** for many use cases.

Rust’s standard library provides **`mpsc`** (multiple producer, single consumer) channels to send messages between threads.

---

## 🔗 What is a Channel?

A channel has two parts:
- `Sender`: used to send data.
- `Receiver`: used to receive data.

```rust
let (sender, receiver) = mpsc::channel();
````

You can `send()` from one thread and `recv()` from another.

---

## 🧪 Example 1: Basic Message Passing

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let message = receiver.recv().unwrap();
        println!("Receiver message: {}", message);
    });

    let message = "Hello, World!".to_string();
    sender.send(message).unwrap();

    handle.join().unwrap();
}
```

### 🔍 What’s Happening:

* A message (`"Hello, World!"`) is created and sent from the **main thread**.
* The **spawned thread** receives it using `.recv()`.

---

## 🧪 Example 2: Message as a Number

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut message: String = receiver.recv().unwrap();
        let mut number: u32 = message.parse().expect("Failed!");

        number += 15;
        message = number.to_string();
        println!("Receiver message: {}", message);
    });

    let message = "45".to_string();
    sender.send(message).unwrap();

    handle.join().unwrap();
}
```

### 🧠 What's New:

* The main thread sends `"45"` as a string.
* The receiving thread parses it, adds `15`, and prints `60`.

---

## 🧪 Example 3: Sending Multiple Messages Over Time

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            "We".to_string(),
            "are".to_string(),
            "strong".to_string(),
            "developers".to_string(),
        ];
        for message in messages {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("{}", received);
    }
}
```

### ⏱️ What's New:

* The sender sends a series of messages with a 1-second delay between each.
* The main thread receives and prints each one.

---

## 🛠 Key Concepts

| Concept                   | Description                                      |
| ------------------------- | ------------------------------------------------ |
| `mpsc::channel()`         | Creates a new channel (sender, receiver)         |
| `sender.send()`           | Sends a message to the receiver                  |
| `receiver.recv()`         | Blocks until a message is received               |
| `for message in receiver` | Automatically receives each message in a loop    |
| `thread::sleep()`         | Pauses thread execution (simulate delay)         |
| `move` in closure         | Transfers ownership of variables into the thread |

---

## 🧱 When to Use Messaging Instead of Mutex?

| Use Case                            | Preferred Method |
| ----------------------------------- | ---------------- |
| Threads write to shared data        | Mutex            |
| Threads communicate by passing data | Channel (mpsc)   |
| Event-driven or producer-consumer   | Channel (mpsc)   |

---

## 🧹 Error Handling Tip

Always `.unwrap()` or use `.expect()` cautiously. In real applications, consider handling errors gracefully:

```rust
if let Ok(message) = receiver.recv() {
    println!("Received: {}", message);
} else {
    eprintln!("Failed to receive message!");
}
```

---

## 📚 Further Reading

* 📘 [Rust Book - Channels](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
* 📖 [`std::sync::mpsc` documentation](https://doc.rust-lang.org/std/sync/mpsc/index.html)

---

## ✅ Summary

| Feature       | Explanation                                   |
| ------------- | --------------------------------------------- |
| `mpsc`        | Stands for multiple producer, single consumer |
| `.send()`     | Send data from one thread                     |
| `.recv()`     | Receive data in another thread                |
| `for in` loop | Receive multiple messages seamlessly          |

---

🧵 Channels are a **powerful and ergonomic way** to implement safe concurrency in Rust. Instead of fighting over shared memory, you pass the data—clean and race-free!


---
