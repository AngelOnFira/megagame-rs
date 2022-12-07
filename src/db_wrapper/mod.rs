use sea_orm::DatabaseConnection;

use crate::task_runner::tasks::TaskType;
#[derive(Debug, Clone)]
pub struct DBWrapper {
    db: DatabaseConnection
}

impl DBWrapper {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn add_task(&self, task: TaskType) {
        // Add a task to the database
    }
}