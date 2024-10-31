---

# Task Manager CLI Application

A basic command-line task management application built with Rust. This tool allows users to add, list, mark as complete, and delete tasks, making it easier to manage daily tasks from the terminal.

Created as a simple practice project.

## Features
- **Add Task**: Create a new task with a description and due date.
- **List Tasks**: Display all tasks with their status (pending or completed).
- **Complete Task**: Mark a task as completed.
- **Delete Task**: Remove a task from the list.
- **Persistent Storage**: Tasks are saved in a JSON file (`tasks.json`) and loaded each time the program runs.

## Prerequisites
A basic understanding of Rust is preferential, but not required.
- [Rust](https://www.rust-lang.org/) (latest stable version recommended)

## Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/task-manager.git
   cd task-manager
   ```

2. **Install dependencies**:
   The project uses `clap` for command-line argument parsing and `serde` for JSON handling. Rust's package manager, Cargo, will automatically install these dependencies when you build or run the project.

3. **Build the project**:
   ```bash
   cargo build --release
   ```

## Usage

### Running the Task Manager
You can run the Task Manager directly from the command line using:
```bash
cargo run -- [command] [arguments]
```

### Commands
#### 1. Add a Task
Add a new task with a description and due date:
```bash
cargo run -- add "Task description" "YYYY-MM-DD"
```
Example:
```bash
cargo run -- add "Finish Rust CLI project" "2024-11-01"
```

#### 2. List Tasks
List all tasks, showing their index, description, due date, and completion status:
```bash
cargo run -- list
```

#### 3. Complete a Task
Mark a task as completed by specifying its index:
```bash
cargo run -- complete [index]
```
Example:
```bash
cargo run -- complete 0
```

#### 4. Delete a Task
Delete a task by specifying its index:
```bash
cargo run -- delete [index]
```
Example:
```bash
cargo run -- delete 0
```

## Task Storage
All tasks are saved in a file called `tasks.json` in the project's root directory. The file is updated each time a task is added, marked as complete, or deleted, providing persistent storage between program runs.

## Example Workflow
1. **Add a task**:
   ```bash
   cargo run -- add "Submit project report" "2024-11-05"
   ```
2. **List tasks**:
   ```bash
   cargo run -- list
   ```
3. **Complete a task**:
   ```bash
   cargo run -- complete 0
   ```
4. **Delete a task**:
   ```bash
   cargo run -- delete 0
   ```

## Potential Future Improvements (To add at own will)
- Automatically add timestamps to tasks.
- Allow users to edit tasks.
- Allow ability to priotize tasks (e.g., high, medium, low).
- Allow users to set reminders for tasks.
- Allow users to set recurring tasks (e.g., daily, weekly, monthly).
- Allow users to set task categories (e.g., work, personal, health).

## Warning!
tasks.json is defaulted to be ignored by Git for security purposes (In case you don't want tasks being made public). If you want to add tasks to the repository, you will need to remove the .gitignore file and commit the changes.

---
