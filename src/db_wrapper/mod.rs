use std::ops::Deref;

use entity::entities::{category, channel, role, task};
use sea_orm::{prelude::*, Database, Set};
use serde::{Deserialize, Serialize};

use crate::task_runner::tasks::{DatabaseId, TaskType};
#[derive(Debug, Clone)]
pub struct DBWrapper {
    pub db: DatabaseConnection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskResult {
    Pending,
    Completed(TaskReturnData),
    // Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskReturnData {
    ChannelModel(channel::Model),
    CategoryModel(category::Model),
    TeamId(DatabaseId),
    UserId(DatabaseId),
    RoleModel(role::Model),
    MessageId(DatabaseId),
    None,
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

    pub async fn add_task(&self, task: TaskType) -> DatabaseId {
        // Add a task to the database
        DatabaseId(
            task::ActiveModel {
                payload: Set(serde_json::to_string(&task).unwrap()),
                status: Set(serde_json::to_string(&TaskResult::Pending).unwrap()),
                ..Default::default()
            }
            .insert(&self.db)
            .await
            .unwrap()
            .id,
        )
    }

    /// Waits for a task to be completed. This will poll the database once a
    /// second, and will cause a deadlock if the task is never completed.
    pub async fn await_task(&self, id: DatabaseId) -> TaskResult {
        async fn check_progress(id: DatabaseId, db: &DatabaseConnection) -> TaskResult {
            // Check if the task is completed
            serde_json::from_str(
                &task::Entity::find_by_id(id.0)
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap()
                    .status,
            )
            .unwrap()
        }

        loop {
            let status = check_progress(id, &self.db).await;
            // If it's not pending, return
            let TaskResult::Pending = status else {
                return status;
            };

            // Sleep for 1 second
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    /// Adds a task to the database and waits for it to be completed
    pub async fn add_await_task(&self, task: TaskType) -> TaskResult {
        let id = self.add_task(task).await;
        self.await_task(id).await
    }
}

impl Deref for DBWrapper {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
