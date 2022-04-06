use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serenity::client::Context;

use sea_orm::{entity::*, query::*};
use tracing::log;

use crate::{
    schema::tasks_task,
    tasks::task::{MessageUser, Task, TaskType},
};

pub mod task;

pub struct TaskRunner {
    pub ctx: Arc<Context>,
    pub db: DatabaseConnection,
}

impl TaskRunner {
    pub async fn run_tasks(&self) {
        // Iterate through open tasks in the DB
        let incomplete_tasks: Vec<tasks_task::Model> = match tasks_task::Entity::find()
            .filter(tasks_task::Column::Completed.eq("false"))
            .all(&self.db)
            .await
        {
            Ok(tasks) => tasks,
            Err(why) => panic!("Error getting tasks: {:?}", why),
        };

        for db_task in incomplete_tasks {
            let task_payload: Task = serde_json::from_str(&db_task.payload).unwrap();

            log::info!("Working on task: {:?}", task_payload);

            // Complete the tasks
            match task_payload.task {
                TaskType::MessageUser(_) => task_payload.message_user(Arc::clone(&self.ctx)).await,
                _ => unimplemented!(),
            }

            // Set the task as completed
            let mut db_task_active_model: tasks_task::ActiveModel = db_task.into();
            db_task_active_model.completed = Set("true".to_string());
            db_task_active_model.update(&self.db).await.unwrap();
        }
    }

    pub async fn sample_tasks(&self) {
        let task = Task {
            task: TaskType::MessageUser(MessageUser {
                player_id: 133358326439346176,
                message: String::from("Good day"),
            }),
        };

        tasks_task::ActiveModel {
            payload: Set(serde_json::to_string(&task).unwrap()),
            completed: Set("true".to_string()),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .unwrap();
        log::info!("Task inserted");
    }
}

// // Add some tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     // fn get_task_runner

//     // #[test]
//     fn test_send_message() {
//         let task = Task {
//             task: TaskType::MessageUser(MessageUser {
//                 player_id: 133358326439346176,
//                 message: String::from("Good day"),
//             }),
//         };

//         tasks_task::ActiveModel {
//             payload: Set(serde_json::to_string(&task).unwrap()),
//             completed: Set("true".to_string()),
//             ..Default::default()
//         }
//         .insert(&self.db)
//         .await
//         .unwrap();
//     }
// }
