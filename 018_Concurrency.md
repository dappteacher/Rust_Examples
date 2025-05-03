# 🧵 Concurrency in Rust

Rust provides powerful, **safe, and fearless concurrency** without data races. Thanks to its ownership model and strong type system, concurrent programming in Rust is efficient and memory-safe at compile time.

---

## 🧠 What is Concurrency?

Concurrency means executing multiple tasks at the same time. Rust supports multiple ways to write concurrent programs:

- **Threads** (`std::thread`)
- **Channels** (`std::sync::mpsc`)
- **Shared state** with locks (`Arc`, `Mutex`)
- **Async/Await** (covered in async Rust)

---

## 🚀 Spawning Threads

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..3 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(200));
    }

    handle.join().unwrap();
}
````

### ✅ Explanation

* `thread::spawn` creates a new thread.
* `join()` waits for the spawned thread to finish.

---

## 📬 Communicating Between Threads with Channels

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec!["Hi", "from", "the", "thread"];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

### ✅ Explanation

* `mpsc` = multiple producer, single consumer.
* `tx.send(val)` sends a message.
* `rx.recv()` or loop over `rx` to receive messages.

---

## 🔐 Shared State with `Mutex` and `Arc`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### ✅ Explanation

* `Mutex<T>` provides safe interior mutability across threads.
* `Arc<T>` is a thread-safe reference-counted smart pointer.
* Always lock a `Mutex` before accessing the data.

---

## 🕒 What is Fearless Concurrency?

Rust eliminates:

* Data races
* Invalid memory access
* Sharing issues without synchronization

Thanks to:

* Ownership system
* Borrow checker
* Safe abstractions for concurrency

---

## ❌ Common Mistakes

| Mistake                   | Fix                                             |
| ------------------------- | ----------------------------------------------- |
| Data race                 | Use `Mutex` or channels                         |
| Use `Rc` instead of `Arc` | Use `Arc` for multi-threaded reference counting |
| Lock not released         | Ensure `lock()` is properly scoped or dropped   |

---

## 🔁 When to Use What?

| Technique       | Use When…                                              |
| --------------- | ------------------------------------------------------ |
| `thread::spawn` | You need true OS threads                               |
| `mpsc::channel` | You want message passing                               |
| `Mutex<T>`      | You share mutable data across threads                  |
| `Arc<T>`        | You need shared ownership across threads               |
| `async/await`   | You do IO-bound or highly concurrent tasks (next step) |

---

## 🧪 Example: Parallel Sum

```rust
use std::thread;

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = arr.split_at(arr.len() / 2);

    let handle = thread::spawn(move || -> i32 {
        right.iter().sum()
    });

    let left_sum: i32 = left.iter().sum();
    let right_sum = handle.join().unwrap();

    println!("Total Sum: {}", left_sum + right_sum);
}
```

---

## 🧠 Summary

| Concept         | Description                                |
| --------------- | ------------------------------------------ |
| `thread::spawn` | Run tasks concurrently                     |
| `mpsc::channel` | Communicate between threads (send/recv)    |
| `Mutex<T>`      | Mutability across threads (guarded access) |
| `Arc<T>`        | Shared ownership in concurrency            |
| `join()`        | Wait for a thread to finish                |

---

## 📚 Further Reading

* [Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
* [Rust by Example - Concurrency](https://doc.rust-lang.org/rust-by-example/std_misc/concurrency.html)

---

💡 Mastering concurrency in Rust means writing faster, parallel, and **safe** code—without fear of subtle bugs. Happy threading!

---

