use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serenity::client::Context;

use sea_orm::{entity::*, query::*};
use tracing::log;

use crate::{
    schema::tasks_task,
    task_runner::tasks::{message_user::MessageUser, TaskType},
};

use self::task_queue::TaskQueue;

pub mod task_queue;
pub mod tasks;

pub struct TaskRunner {
    pub ctx: Arc<Context>,
    pub db: Box<dyn TaskQueue + Send + Sync>,
}

impl TaskRunner {
    pub async fn run_tasks(&self) {
        // for db_task in incomplete_tasks {
        //     let task_payload: TaskType = serde_json::from_str(&db_task.payload).unwrap();

        //     log::info!("Working on task: {:?}", task_payload);

        //     // Complete the tasks
        //     let task = task_payload.route().handle(Arc::clone(&self.ctx));
        //     task.await;

        //     // Set the task as completed
        //     let mut db_task_active_model: tasks_task::ActiveModel = db_task.into();
        //     db_task_active_model.completed = Set("true".to_string());
        //     db_task_active_model.update(&self.db).await.unwrap();
        // }
    }

    pub async fn _sample_tasks(&mut self) {
        let task = TaskType::MessageUser(MessageUser {
            player_id: 133358326439346176,
            message: String::from("Good day"),
        });

        self.db.add_task(task).await;
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
