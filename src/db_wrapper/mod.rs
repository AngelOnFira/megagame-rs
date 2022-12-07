use std::ops::Deref;

use entity::entities::task;
use sea_orm::{prelude::*, Set};

use crate::task_runner::tasks::TaskType;
#[derive(Debug, Clone)]
pub struct DBWrapper {
    pub db: DatabaseConnection,
}

impl DBWrapper {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn add_task(&self, task: TaskType) {
        // Add a task to the database
        task::ActiveModel {
            payload: Set(serde_json::to_string(&task).unwrap()),
            completed: Set(false),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .unwrap();
    }
}

impl Deref for DBWrapper {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
