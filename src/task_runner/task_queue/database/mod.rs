use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::log;

use crate::{schema::tasks_task, task_runner::tasks::TaskType};
use sea_orm::ColumnTrait;

use super::TaskQueue;

pub struct MemoryTaskQueue {
    pub db: DatabaseConnection,
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn get_available_tasks(&mut self) -> Vec<TaskType> {
        // Iterate through open tasks in the DB
        let incomplete_tasks: Vec<tasks_task::Model> = match tasks_task::Entity::find()
            .filter(tasks_task::Column::Completed.eq("false"))
            .all(&self.db)
            .await
        {
            Ok(tasks) => tasks,
            Err(why) => panic!("Error getting tasks: {:?}", why),
        };

        incomplete_tasks
            .iter()
            .map(|task| serde_json::from_str(&task.payload).unwrap())
            .collect()
    }
}
