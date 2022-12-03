use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::log;

use crate::{schema::tasks_task, task_runner::tasks::TaskType};
use sea_orm::ColumnTrait;

use super::TaskQueue;

pub struct DatabaseTaskQueue {
    pub db: DatabaseConnection,
}

#[async_trait]
impl TaskQueue for DatabaseTaskQueue {
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

    async fn add_task(&mut self, task: TaskType) {
        tasks_task::ActiveModel {
            payload: Set(serde_json::to_string(&task).unwrap()),
            completed: Set("false".to_string()),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .unwrap();
        log::info!("Task inserted");
    }
}
