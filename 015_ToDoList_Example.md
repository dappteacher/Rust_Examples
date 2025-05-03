# 📝 To-Do List Manager in Rust

A simple command-line To-Do List Manager written in Rust.

---

## 🛠️ Concepts Covered

- ✅ Structs (`struct`)
- ✅ Vectors (`Vec<T>`)
- ✅ Enums (`enum`) *(optional for future expansion)*
- ✅ User Input (`io::stdin`)
- ✅ Match Statements (`match`)

---

## 📜 How It Works

1. Users can **add**, **view**, **complete**, and **delete** tasks.
2. Tasks are stored in-memory using a `Vec<Task>`.
3. The interface is menu-driven, using text input.

---

## 📦 Project Structure

```

todo\_list/
└── src/
└── main.rs

````

---

## 🔧 `src/main.rs`
```rust
use std::io;

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            done: false,
        }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n📌 To-Do List Manager");
        println!("1️⃣  Add Task");
        println!("2️⃣  View Tasks");
        println!("3️⃣  Mark Task as Done");
        println!("4️⃣  Delete Task");
        println!("5️⃣  Exit");

        let choice = get_user_input("Choose an option:");

        match choice.trim() {
            "1" => {
                let desc = get_user_input("Enter task description:");
                if desc.is_empty() {
                    println!("⚠ Task description cannot be empty!");
                    continue;
                }
                tasks.push(Task::new(next_id, desc));
                println!("✅ Task {} added!", next_id);
                next_id += 1;
            }

            "2" => {
                if tasks.is_empty() {
                    println!("📭 No tasks found.");
                } else {
                    println!("\n📋 Task List:");
                    for task in &tasks {
                        let status = if task.done { "✔ Done" } else { "❌ Pending" };
                        println!("{}: {} [{}]", task.id, task.description, status);
                    }
                }
            }

            "3" => {
                let id = parse_id("Enter task ID to mark as done:");
                match tasks.iter_mut().find(|t| t.id == id) {
                    Some(task) => {
                        task.mark_done();
                        println!("🎉 Task {} marked as done!", id);
                    }
                    None => println!("⚠ Task not found."),
                }
            }

            "4" => {
                let id = parse_id("Enter task ID to delete:");
                if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                    tasks.remove(pos);
                    println!("🗑️ Task {} deleted!", id);
                } else {
                    println!("⚠ Task not found.");
                }
            }

            "5" => {
                println!("👋 Exiting... Have a great day!");
                break;
            }

            _ => println!("⚠ Invalid choice. Please try again."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn parse_id(prompt: &str) -> usize {
    loop {
        let input = get_user_input(prompt);
        match input.trim().parse::<usize>() {
            Ok(id) => return id,
            Err(_) => println!("⚠ Please enter a valid number."),
        }
    }
}
````

---

## ✅ Features

* Add a new task (`1`)
* View all tasks (`2`)
* Mark a task as done (`3`)
* Delete a task (`4`)
* Exit the program (`5`)

---

## 🚀 How to Run

```bash
# 1. Create a new Rust project
cargo new todo_list
cd todo_list

# 2. Replace src/main.rs with the code above

# 3. Run the program
cargo run
```

---

## 💡 Possible Enhancements

* 💾 Save/load tasks to/from a file using `serde` and `serde_json`.
* 📆 Add deadlines or categories to tasks.
* 🧑‍💻 Convert to a GUI using libraries like `egui` or `iced`.
* 🌐 Build a web version using `Yew` (Rust frontend framework).

---

> 🦀 Perfect for beginners who want to explore practical Rust projects or educators introducing Rust’s struct, input handling, and ownership model.

---
