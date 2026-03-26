1. Title & Objective
Title: "Rust-Powered Productivity: Building a Multi-Threaded CLI Task Manager"

Technology: Rust Programming Language.

Why: I chose Rust for its memory safety guarantees and its growing importance in the Bitcoin and Blockchain ecosystem.

Goal: To create a CLI app that uses background threads to send real-time desktop reminders.

2. Quick Summary
What is it? Rust is a systems programming language focused on safety, speed, and concurrency.

Where is it used? It’s used in high-performance engines (like Firefox), cloud infrastructure (AWS), and secure financial protocols (Bitcoin Lightning Network).

Real-world Example: The Polkadot and Solana blockchains are built primarily in Rust.

3. System Requirements
OS: Linux (Tested on Kali/Ubuntu), Windows, or macOS.

Tools: VS Code (with Rust-Analyzer extension).

Packages: rustup (Rust toolchain installer).

4. Installation & Setup
Install Rust: ```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Linux Dependencies (for notifications):
 ```bash
sudo apt update && sudo apt install libdbus-1-dev pkg-config
```
3.Initialize Project:
```bash
cargo new rust_todo
cd rust_todo
```
5. Minimal Working Example

This example demonstrates how Rust uses an Arc (Atomic Reference Counter) and a Mutex (Mutual Exclusion) to allow a background thread and a main thread to access the same task list safely.

``` use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create a shared, thread-safe list of tasks
    // Arc allows multiple owners; Mutex allows safe modification
    let tasks = Arc::new(Mutex::new(vec!["Finish Capstone", "Submit to GitHub"]));

    // 2. Clone the pointer for the background thread
    let bg_tasks = Arc::clone(&tasks);

    // 3. Spawn a background thread to "monitor" tasks
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let list = bg_tasks.lock().unwrap();
            println!("\n[Background Monitor] Current task count: {}", list.len());
        }
    });

    // 4. Main thread: Add a new task
    {
        let mut list = tasks.lock().unwrap();
        println!("[Main Thread] Adding a new task...");
        list.push("Celebrate 5/5 Grade");
    }

    // Keep the program running for a moment to see the background thread work
    thread::sleep(Duration::from_secs(6));
}
```
How it works:
Arc: This is a smart pointer that lets the background thread and the main thread both "own" the task list.

Mutex: This acts like a lock. Before the background thread can read the list, it must lock() it. This prevents the program from crashing if both threads try to touch the data at the exact same time.

Expected Output
When you run the code above using cargo run, the terminal will display:

```
[Main Thread] Adding a new task...
[Background Monitor] Current task count: 3
```
6. AI Prompt Journal

Prompt Used,AI Response Summary,Evaluation
"""How to add a / after src..i just created a folder named src""",Explained the difference between a file and a directory in Linux.,Helpful: Corrected a basic terminal mistake.
"""how to send desktop notifications in Rust on Linux""",Suggested the notify-rust crate and provided the code structure.,Excellent: Introduced a new library I hadn't used.
"""how to share data between a background thread and the main loop""",Explained Arc and Mutex for safe memory sharing.,Vital: This was the most complex part of the learning.

7. Common Issues & Fixes
Error: Path 'src' is not a directory.

Fix: I accidentally created a file named src. I had to rm src and then mkdir src.

Error: Command 'cargo' not found.

Fix: Rust wasn't in the system PATH. I used source "$HOME/.cargo/env" to refresh the terminal environment.

8. References
Official Rust Book

Crates.io (Package Registry)

Rust-Analyzer Documentation

















