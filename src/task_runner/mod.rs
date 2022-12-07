use std::sync::Arc;

use entity::entities::task;
use sea_orm::{prelude::*, Set};
use serenity::client::Context;
use tracing::log;

use crate::{db_wrapper::DBWrapper, task_runner::tasks::TaskType};

use self::tasks::message_user::MessageUser;

pub mod tasks;

pub struct TaskRunner {
    pub ctx: Arc<Context>,
    pub db: DBWrapper,
}

impl TaskRunner {
    pub async fn run_tasks(&self) {
        // Get all the incomplete tasks from the database
        let incomplete_tasks: Vec<task::Model> = match task::Entity::find()
            .filter(task::Column::Completed.eq(false))
            .all(&*self.db)
            .await
        {
            Ok(tasks) => tasks,
            Err(why) => panic!("Error getting tasks: {:?}", why),
        };

        // Print the tasks
        log::info!("Found {} tasks", incomplete_tasks.len());

        // Iterate through open tasks in the DB
        for db_task in incomplete_tasks {
            let task_payload: TaskType = serde_json::from_str(&db_task.payload).unwrap();

            log::info!("Working on task: {:?}", task_payload);

            // Complete the tasks
            let task = task_payload.route().handle(Arc::clone(&self.ctx));
            task.await;

            // Set the task as completed
            let mut db_task_active_model: task::ActiveModel = db_task.into();
            db_task_active_model.completed = Set(true);
            db_task_active_model.update(&*self.db).await.unwrap();
        }
    }

    // pub async fn sample_tasks(&mut self) {
    //     let task = TaskType::MessageUser(MessageUser {
    //         player_id: 133358326439346176,
    //         message: String::from("Good day"),
    //     });

    //     self.db.add_task(task).await;
    // }
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
