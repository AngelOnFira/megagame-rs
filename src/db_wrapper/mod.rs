use std::ops::Deref;

use sea_orm::DatabaseConnection;

use crate::task_runner::tasks::TaskType;
#[derive(Debug, Clone)]
pub struct DBWrapper {
    pub db: DatabaseConnection,
}

impl DBWrapper {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn add_task(&self, task: TaskType) {
        // Add a task to the database
    }
}

impl Deref for DBWrapper {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
