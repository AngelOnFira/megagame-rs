use async_trait::async_trait;

use super::tasks::TaskType;

mod database;
mod memory;

#[async_trait]
pub trait TaskQueue {
    async fn get_available_tasks(&mut self) -> Vec<TaskType>;

    async fn add_task(&mut self, task: TaskType);
}
