use chrono::{Local, NaiveDateTime};
use colored::*;
use notify_rust::Notification; // New: For Desktop Popups
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tabled::{Table, Tabled};

#[derive(Serialize, Deserialize, Tabled, Clone)]
struct Task {
    #[tabled(rename = "ID")]
    id: usize,
    #[tabled(rename = "Task")]
    title: String,
    #[tabled(rename = "Description")]
    description: String, // New Field
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Reminder")]
    reminder: String,
}

struct TodoList {
    tasks: Vec<Task>,
    file_path: String,
}

impl TodoList {
    fn load(path: &str) -> Self {
        let tasks = fs::read_to_string(path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_else(Vec::new);
        TodoList { tasks, file_path: path.to_string() }
    }

    fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.file_path, json)
    }

    fn add(&mut self, title: String, description: String, reminder_raw: String) {
        let id = self.tasks.len() + 1;
        let new_task = Task {
            id,
            title,
            description,
            status: "Pending".to_string(),
            reminder: if reminder_raw.is_empty() { "None".to_string() } else { reminder_raw },
        };
        self.tasks.push(new_task);
        let _ = self.save();
    }

    fn edit(&mut self, id: usize, new_title: String, new_desc: String) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if !new_title.is_empty() { task.title = new_title; }
            if !new_desc.is_empty() { task.description = new_desc; }
            let _ = self.save();
            return true;
        }
        false
    }

    fn toggle_status(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = if task.status == "Pending" { "Completed".green().to_string() } else { "Pending".to_string() };
            let _ = self.save();
            return true;
        }
        false
    }

    fn remove(&mut self, id: usize) -> bool {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        let _ = self.save();
        self.tasks.len() < original_len
    }
}

fn main() {
    let todo_list = Arc::new(Mutex::new(TodoList::load("tasks.json")));
    let list_for_thread = Arc::clone(&todo_list);

    // BACKGROUND THREAD: Now sends DESKTOP POPUPS
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30)); // Check every 30s
            let list = list_for_thread.lock().unwrap();
            let now = Local::now().format("%Y-%m-%d %H:%M").to_string();
            
            for task in &list.tasks {
                if task.reminder == now && !task.status.contains("Completed") {
                    // DESKTOP NOTIFICATION
                    Notification::new()
                        .summary("📌 Task Reminder")
                        .body(&format!("{}: {}\nOpen CLI to mark as Done!", task.title, task.description))
                        .icon("dialog-information")
                        .show()
                        .unwrap();
                }
            }
        }
    });

    loop {
        println!("\n{}", "=== RUST TASK MANAGER PRO ===".bright_magenta().bold());
        println!(" Add List Done Edit Delete Exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                let title = prompt("Title: ");
                let desc = prompt("Description: ");
                let remind = prompt("Remind (YYYY-MM-DD HH:MM): ");
                todo_list.lock().unwrap().add(title, desc, remind);
            }
            "2" => {
                let list = todo_list.lock().unwrap();
                println!("{}", Table::new(&list.tasks).to_string());
            }
            "3" => {
                let id: usize = prompt("Task ID to complete: ").parse().unwrap_or(0);
                todo_list.lock().unwrap().toggle_status(id);
            }
            "4" => {
                let id: usize = prompt("Task ID to edit: ").parse().unwrap_or(0);
                let title = prompt("New Title (leave blank to keep): ");
                let desc = prompt("New Description (leave blank to keep): ");
                todo_list.lock().unwrap().edit(id, title, desc);
            }
            "5" => {
                let id: usize = prompt("Task ID to delete: ").parse().unwrap_or(0);
                todo_list.lock().unwrap().remove(id);
            }
            "6" => break,
            _ => println!("Invalid option."),
        }
    }
}

fn prompt(query: &str) -> String {
    print!("{}", query);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
