# 🦀 Rust Task Manager Pro
**High-Performance CLI with Desktop Notifications**
*Developed by Mildred Embeywa*

A cross-platform Task Management system built in Rust. This application features a multi-threaded architecture to handle real-time desktop reminders while allowing a smooth user experience in the terminal.

---

## 🚀 Features
- **Smart Reminders:** Integrated background thread monitors tasks and sends **System Desktop Popups** (Linux/Ubuntu/Windows).
- **Persistent Storage:** Tasks are saved to `tasks.json` automatically—no data is lost on exit.
- **Rich Interface:** Color-coded status updates and formatted data tables.
- **Task Metadata:** Supports Titles, detailed Descriptions, and specific Timestamps.

---

## 🛠️ Installation & Setup

### For Linux & Ubuntu Users
1. **Install Dependencies:**
   Open your terminal and run:
   ```bash
   sudo apt update && sudo apt install libdbus-1-dev pkg-config
```

2.Build and Run:
```bash
cargo run
```
For Windows Users
Install Rust: Download the installer from rustup.rs.

Run the App: Open PowerShell or Command Prompt in the project folder and type:
 ```bash
cargo run
```

For Visual Studio Code (All Platforms)
Open the project folder in VS Code.

Install the Rust-Analyzer extension from the Marketplace.

Open src/main.rs and click the "Run" button that appears above the main function, or press Ctrl + ` to use the integrated terminal.



**Technical Deep Dive (For Reviewers)*
This project was designed to demonstrate "Idiomatic Rust" and systems-level safety:

Concurrency: Uses std::thread to run a background monitor without blocking the main UI.

Thread Safety: Utilizes Arc<Mutex<T>> (Atomic Reference Counting and Mutual Exclusion) to safely share task data between threads.

Serialization: Uses the Serde framework to translate Rust structs into JSON format.

Memory Safety: Zero use of unsafe code or .unwrap(), ensuring the app handles errors gracefully rather than crashing.

** Project Structure**
src/main.rs: The core engine, data models, and UI logic.

Cargo.toml: Project metadata and library dependencies.

tasks.json: Your local database (auto-created on first run).
