# Blockless_rust_sdk
"A Rust SDK for managing and executing WebAssembly modules on the Blockless platform"

## Overview
This SDK allows you to build and execute WebAssembly (WASM) modules, manage execution flow, and interact with external APIs via HTTP requests. It is designed to integrate with the Blockless platform, offering capabilities such as memory management, task prioritization, parallel processing, and safe data handling.

## Features
- **Load and Execute WASM Modules**: Load WASM modules into memory, invoke exported functions, and interact with the module's internal state.
- **Manage Execution Flow**: Prioritize tasks, handle parallel processing, and manage execution flow to optimize performance.
- **HTTP Requests**: Send and receive HTTP requests to integrate with external APIs or services.
- **Memory Management**: Access and modify the memory of WASM modules for state management and computation.
- **Handle Long-Running Tasks**: Execute tasks that run asynchronously or in parallel within WASM modules.

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo (Rust's package manager)
- WABT tools for validating WASM files

### Installation

To use the Blockless Rust SDK in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
blockless-rust-sdk = "0.1.0"

# WebAssembly support and serialization
wasmtime = "0.30.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

# For parallel processing and async support
tokio = { version = "1", features = ["full"] }
async-std = "1.10"

# HTTP client
reqwest = { version = "0.11", features = ["blocking", "json"] }

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
getrandom = { version = "0.2", features = ["js"] }
```

## User Guide

## 1. Building the project
To get started, clone the repository and set up the project dependencies.

```bash
git clone https://github.com/arjunkotgire/blockless-rust-sdk
cd blockless-rust-sdk
cargo build 
```

## 2. Setup Instructions

To set up the environment, you need to install the necessary dependencies listed in the `Cargo.toml` file. This includes support for WebAssembly, HTTP requests, and asynchronous processing.

```bash
cargo install --path .
```

## 3. Usage Instructions

### Loading a WASM Module

You can load a WASM module by providing the binary data to the SDK. Here's an example of how to load a WASM module:

```rust
use blockless_rust_sdk::wasm::load_wasm_module;
use std::fs::File;
use std::io::Read;

fn main() {
    let wasm_file_path = "path/to/your/wasm/file.wasm";
    let mut file = File::open(wasm_file_path).expect("Failed to open WASM file");
    let mut wasm_bytes = Vec::new();
    file.read_to_end(&mut wasm_bytes).expect("Failed to read WASM file");

    let module = load_wasm_module(&wasm_bytes).expect("Failed to load WASM module");
    // Now you can invoke functions, read/write memory, etc.
}
```

### Managing Execution Flow

The SDK allows you to manage execution flow with task prioritization and parallel processing:

```rust
use blockless_rust_sdk::execution::ExecutionFlow;

#[tokio::main]
async fn main() {
    let execution_flow = ExecutionFlow::new(4); // Create an ExecutionFlow with 4 threads
    execution_flow.add_task(1, "Low priority task".to_string());
    execution_flow.add_task(5, "High priority task".to_string());

    execution_flow.manage_flow().await; // This will manage and process the tasks
}
```

### Making HTTP Requests

You can make HTTP GET and POST requests using the SDK's HTTP module:

```rust
use blockless_rust_sdk::http::BlocklessAPI;

#[tokio::main]
async fn main() {
    let api = BlocklessAPI::new("https://example.com");

    // Example GET request
    let response = api.get("endpoint").await.expect("Failed to send GET request");
    println!("GET Response: {}", response);

    // Example POST request
    let response = api.post("endpoint", "{\"key\": \"value\"}").await.expect("Failed to send POST request");
    println!("POST Response: {}", response);
}
```

## 4. Test
The SDK includes unit and integration tests to ensure functionality. To run the tests, use the following command:
```
cargo test
```

## 5. Check list (updating..)

- Ensure the path to the WASM file is correct and that the file is accessible.
- Confirm that the WASM file is valid and compiled correctly.
- Use tools like `wasm-objdump` to inspect the contents of the WASM module.



## 6. Examples

### Task_manager dApp

The `Task_manager` dApp demonstrates the use of the Blockless Rust SDK to manage tasks with different priorities, execute them in parallel, and interact with external APIs. Below are the steps to build and run this example.

#### Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.
- Ensure you have set the correct dependencies in  `cargo.toml` file (imp). 

#### Cloning and Building

Clone the repository and navigate to the `task_manager` example directory:

```bash
 git clone https://github.com/arjunkotgire/blockless-rust-sdk.git
 cd blockless-rust-sdk/examples/task_manager
```

Use Cargo build to build the TaskManager dApp:

```bash
cargo build 
cargo run
```

### TaskManager dApp - `main.rs` Overview

#### 1. Task Definition

The `Task` structure defines the properties of a task:

```rust
#[derive(Clone)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {
    fn new(description: &str, priority: u8) -> Self {
        Task {
            description: description.to_string(),
            priority,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}
```

#### 2. TaskManager Structure

Manages tasks and integrates with the Blockless API:

```rust
struct TaskManager {
    tasks: Arc<Mutex<HashMap<u32, Task>>>,
    execution_flow: ExecutionFlow,
    api: Arc<Mutex<BlocklessAPI>>,
}

impl TaskManager {
    fn new(threads: usize, api_url: &str) -> Self {
        TaskManager {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            execution_flow: ExecutionFlow::new(threads),
            api: Arc::new(Mutex::new(BlocklessAPI::new(api_url))),
        }
    }
}
```

#### 3. Adding a Task

Adds a task and schedules it based on priority:

```rust
async fn add_task(&self, id: u32, description: &str, priority: u8) {
    let mut tasks = self.tasks.lock().await;
    let task = Task::new(description, priority);
    tasks.insert(id, task);
    self.execution_flow.add_task(priority, id.to_string());
}
```

#### 4. Viewing Tasks

Displays all tasks:

```rust
async fn view_tasks(&self) {
    let tasks = self.tasks.lock().await;
    for (id, task) in tasks.iter() {
        println!(
            "Task ID: {}, Description: {}, Priority: {}, Completed: {}",
            id, task.description, task.priority, task.completed
        );
    }
}
```

#### 5. Updating a Task

Updates the details of a specific task:

```rust
async fn update_task(&self, id: u32, description: Option<&str>, priority: Option<u8>) {
    let mut tasks = self.tasks.lock().await;
    if let Some(task) = tasks.get_mut(&id) {
        if let Some(desc) = description {
            task.description = desc.to_string();
        }
        if let Some(prio) = priority {
            task.priority = prio;
        }
    }
}
```

#### 6. Completing a Task

Marks a task as completed and sends an HTTP notification:

```rust
async fn complete_task(&self, id: u32) {
    let mut tasks = self.tasks.lock().await;
    if let Some(task) = tasks.get_mut(&id) {
        task.complete();
        println!("Task ID: {} is completed.", id);

        let notification = format!("Task ID: {} is completed.", id);
        let api = self.api.clone();
        tokio::spawn(async move {
            let api = api.lock().await;
            if let Err(e) = api.post("task/completed", &notification).await {
                eprintln!("Failed to send notification: {}", e);
            }
        });
    }
}
```

#### 7. Executing Tasks

Processes tasks based on priority:

```rust
async fn execute_tasks(&self) {
    self.execution_flow.manage_flow().await;
    println!("All tasks have been executed based on their priority.");
}
```

#### 8. Main Function

Demonstrates `TaskManager` operations:

```rust
#[tokio::main]
async fn main() {
    let task_manager = TaskManager::new(4, "https://example.com/api");

    task_manager.add_task(1, "Implement TaskManager", 5).await;
    task_manager.add_task(2, "Write documentation", 3).await;
    task_manager.add_task(3, "Submit project", 4).await;

    task_manager.view_tasks().await;
    task_manager.update_task(2, Some("Write detailed documentation"), Some(4)).await;
    task_manager.complete_task(1).await;
    task_manager.execute_tasks().await;
    task_manager.view_tasks().await;
}
```
