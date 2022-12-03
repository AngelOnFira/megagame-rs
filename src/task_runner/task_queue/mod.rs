use async_trait::async_trait;

use super::tasks::{DbTask, TaskType};

pub mod database;
pub mod memory;

#[async_trait]
pub trait TaskQueue {
    async fn get_available_tasks(&mut self) -> Vec<DbTask>;

    async fn add_task(&mut self, task: TaskType);

    async fn complete_task(&mut self, task: DbTask);
}
