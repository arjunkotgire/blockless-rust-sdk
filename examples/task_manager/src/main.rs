use tokio::sync::Mutex;
use blockless_rust_sdk::execution::ExecutionFlow;
use blockless_rust_sdk::http::BlocklessAPI;
use std::sync::Arc;
use std::collections::HashMap;

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

    async fn add_task(&self, id: u32, description: &str, priority: u8) {
        let mut tasks = self.tasks.lock().await;
        let task = Task::new(description, priority);
        tasks.insert(id, task);

        self.execution_flow.add_task(priority, id.to_string());
    }

    async fn view_tasks(&self) {
        let tasks = self.tasks.lock().await;
        for (id, task) in tasks.iter() {
            println!(
                "Task ID: {}, Description: {}, Priority: {}, Completed: {}",
                id, task.description, task.priority, task.completed
            );
        }
    }

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

    async fn complete_task(&self, id: u32) {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.get_mut(&id) {
            task.complete();
            println!("Task ID: {} is completed.", id);

            // Send notification via HTTP
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

    async fn execute_tasks(&self) {
        self.execution_flow.manage_flow().await;
        println!("All tasks have been executed based on their priority.");
    }
}

#[tokio::main]
async fn main() {
    let task_manager = TaskManager::new(4, "https://example.com/api");

    task_manager.add_task(1, "Implement TaskManager", 5).await;
    task_manager.add_task(2, "Write documentation", 3).await;
    task_manager.add_task(3, "Submit project", 4).await;

    task_manager.view_tasks().await;

    task_manager.update_task(2, Some("Write detailed documentation"), Some(4)).await;

    task_manager.view_tasks().await;

    task_manager.complete_task(1).await;  // Make sure to await async function

    task_manager.execute_tasks().await;

    task_manager.view_tasks().await;
}
