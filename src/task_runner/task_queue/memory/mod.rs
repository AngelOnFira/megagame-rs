use async_trait::async_trait;

use crate::task_runner::tasks::TaskType;

use super::TaskQueue;

pub struct MemoryTaskQueue {
    pub tasks: Vec<TaskType>,
}

impl MemoryTaskQueue {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn get_available_tasks(&mut self) -> Vec<TaskType> {
        // Return all the tasks, and remove them from the queue
        let tasks = self.tasks.clone();
        self.tasks.clear();
        tasks
    }

    async fn add_task(&mut self, task: TaskType) {
        self.tasks.push(task);
    }
}
