use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    due_date: String,
    completed: bool,
}

fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Manage your tasks from the command line")
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(
                    Arg::new("description")
                        .required(true)
                        .help("Task description"),
                )
                .arg(
                    Arg::new("due_date")
                        .required(true)
                        .help("Due date for the task"),
                ),
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .subcommand(
            Command::new("complete")
                .about("Marks a task as completed")
                .arg(Arg::new("index").required(true).help("Index of the task")),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a task")
                .arg(Arg::new("index").required(true).help("Index of the task")),
        )
        .get_matches();

    let mut tasks: Vec<Task> = load_tasks().unwrap_or_else(|_| vec![]);

    if let Some(matches) = matches.subcommand_matches("add") {
        let description = matches.get_one::<String>("description").unwrap();
        let due_date = matches.get_one::<String>("due_date").unwrap();
        let task = Task {
            description: description.to_string(),
            due_date: due_date.to_string(),
            completed: false,
        };
        tasks.push(task);
        save_tasks(&tasks).expect("Unable to save tasks");
        println!("Task added!");
    } else if matches.subcommand_matches("list").is_some() {
        list_tasks(&tasks);
    } else if let Some(matches) = matches.subcommand_matches("complete") {
        let index: usize = matches.get_one::<String>("index").unwrap().parse().unwrap();
        if let Some(task) = tasks.get_mut(index) {
            task.completed = true;
            save_tasks(&tasks).expect("Unable to save tasks");
            println!("Task marked as completed!");
        } else {
            println!("Task not found!");
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let index: usize = matches.get_one::<String>("index").unwrap().parse().unwrap();
        if index < tasks.len() {
            tasks.remove(index);
            save_tasks(&tasks).expect("Unable to save tasks");
            println!("Task deleted!");
        } else {
            println!("Task not found!");
        }
    }
}

fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    let path = Path::new("tasks.json");
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let tasks = serde_json::from_reader(reader)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

fn save_tasks(tasks: &[Task]) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(tasks)?;
    let mut file = File::create("tasks.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { "✗" };
            println!(
                "[{}] {} - Due: {} [{}]",
                index, task.description, task.due_date, status
            );
        }
    }
}
