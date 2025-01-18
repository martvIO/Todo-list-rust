use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, BufWriter, Read, Write}, ops::Index,
};
use colored::Colorize;

struct Todo {
    todo: Vec<String>,
    file_path: String,
}

impl Todo {
    /// Creates a new instance of Todo and loads tasks from a file
    fn new(file_path: &str) -> Self {
        let mut todo = Vec::new();
        if let Ok(contents) = fs::read_to_string(file_path) {
            todo = contents
                .lines()
                .map(|line| line.to_string())
                .collect();
        }
        Todo {
            todo,
            file_path: file_path.to_string(),
        }
    }

    /// Saves the current todo list to a file
    fn save(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .expect("Failed to open file for saving tasks");

        for task in &self.todo {
            writeln!(file, "{}", task).expect("Failed to write task to file");
        }
    }

    /// Adds tasks to the todo list
    pub fn add(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("❌ Not enough arguments to add a task.");
        } else {
            for arg in args {
                let symbol: String = String::from("[ ] ");
                let task: String = symbol + &arg;
                self.todo.push(task);
            }
            println!("✅ Task(s) added successfully!");
            self.save();
        }
    }

    pub fn rm(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("❌ Not enough arguments to add a task.");
        } else {
            let index: usize = args[0].parse().expect("❌ Invalid task number.");
            // Check if the index is valid
            if index < self.todo.len() {
                // Remove the task at the specified index
                self.todo.remove(index-1);
                println!("✅ Task removed successfully!");
                self.save(); // Save the updated list to the file
            } else {
                eprintln!("❌ Task number out of range.");
            }
        }
    }

    pub fn done(&mut self, args: &[String]) {
        if args.is_empty() {
            eprintln!("❌ Not enough arguments to add a task.");
        } else {
            let index: usize = args[0].parse().expect("❌ Invalid task number.");

            if index <= self.todo.len() {
                let task = self.todo.index(index-1).clone().replace("[ ]", "[*]");
                self.todo.remove(index-1);
                println!("{}",task);
                println!("✅ Task '{}' marked as done!", task);
                self.todo.push(task);
                self.save();
            }
        }
    }

    pub fn edit(&mut self, args: &[String]){
        if args.is_empty() {
            eprintln!("❌ Not enough arguments to add a task.");
        } else {
            let index: usize = args[0].parse().expect("❌ Invalid task number.");
            
            let symbol: String = String::from("[ ] ");
            let new_task = symbol + &args[1].to_string();
            println!("{}",new_task);
            self.todo[index-1] = new_task;
            println!("{:?}",self.todo);
            self.save();
        }
    }

    pub fn reset(&mut self) {
        let empty_todo: Vec<String> = Vec::new();
        self.todo = empty_todo;
        self.save();
    }
    
    /// Lists all tasks in the todo list
    pub fn list(&self) {
        if self.todo.is_empty() {
            println!("📭 Your todo list is empty!");
            return;
        }

        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout);

        for (number, task) in self.todo.iter().enumerate() {
            let number = (number + 1).to_string().bold();
            let (symbol, task_content) = if task.len() > 4 {
                (&task[..4], &task[4..])
            } else {
                ("", task.as_str())
            };

            let formatted_task = match symbol {
                "[*] " => format!("{} {}\n", number, task_content.strikethrough()),
                "[ ] " => format!("{} {}\n", number, task_content),
                _ => format!("{} {}\n", number, task),
            };

            writer
                .write_all(formatted_task.as_bytes())
                .expect("Failed to write to stdout");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: String = env::var("TODO_FILE_PATH").unwrap_or("todo_list.txt".to_string());

    let mut todo = Todo::new(&file_path);

    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => todo.rm(&args[2..]),
            "done" => todo.done(&args[2..]),
            "edit" => todo.edit(&args[2..]),
            "reset" => todo.reset(),
            _ => eprintln!("❓ Unknown command: {}", command),
        }
    } else {
        println!("ℹ️ Usage: cargo run [list|add <tasks>]");
    }
}
