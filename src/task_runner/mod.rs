use std::sync::Arc;

use serenity::client::Context;
use tracing::log;

use crate::task_runner::tasks::{message_user::MessageUser, TaskType};

use self::task_queue::TaskQueue;

pub mod task_queue;
pub mod tasks;

pub struct TaskRunner {
    pub ctx: Arc<Context>,
    pub db: Box<dyn TaskQueue + Send + Sync>,
}

impl TaskRunner {
    pub async fn run_tasks(&mut self) {
        for task_payload in self.db.get_available_tasks().await {
            log::info!("Working on task: {:?}", task_payload);

            // Complete the tasks
            let task = task_payload.task.route().handle(Arc::clone(&self.ctx));
            task.await;

            // Set the task as completed
            self.db.complete_task(task_payload).await;
        }
    }

    pub async fn sample_tasks(&mut self) {
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
