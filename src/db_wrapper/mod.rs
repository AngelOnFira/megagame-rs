use std::ops::Deref;

use entity::entities::task;
use sea_orm::{prelude::*, Database, Set};

use crate::task_runner::tasks::TaskType;
#[derive(Debug, Clone)]
pub struct DBWrapper {
    pub db: DatabaseConnection,
}

impl DBWrapper {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn new_default_db() -> Self {
        let db: DatabaseConnection = match Database::connect("sqlite://./db.sqlite3").await {
            Ok(db) => db,
            Err(err) => panic!("Error connecting to database: {:?}", err),
        };

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
