use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serenity::client::Context;

use serenity::{http::CacheHttp, model::id::UserId};

use sea_orm::{entity::*, query::*};

// use self::message::Message;

// mod message;

use crate::schema::tasks_task;

#[derive(Serialize, Deserialize)]
pub enum TaskType {
    MessageUser(MessageUser),
    ChangeTeam,
    CreateRole,
    CreateCategory,
    CreateTeamChannel,
    CreateTeamVoiceChannel,
    CreateCategoryChannel,
    CreateButtons,
    CreateMessage,
    CreateThread,
}

pub struct TaskRunner {
    pub ctx: Arc<Context>,
    pub db: DatabaseConnection,
}

#[derive(Serialize, Deserialize)]
struct Task {
    task: TaskType,
}

#[derive(Serialize, Deserialize)]
pub struct MessageUser {
    player_id: u64,
    message: String,
}

impl Task {
    async fn message_user(&self, ctx: Arc<Context>) {
        let message = if let TaskType::MessageUser(message) = &self.task {
            message
        } else {
            panic!("Not a message task");
        };

        if let Ok(user) = UserId(message.player_id).to_user(ctx.http()).await {
            match user
                .direct_message(ctx.http(), |m| m.content(message.message.as_str()))
                .await
            {
                Ok(_) => println!("Message sent"),
                Err(why) => println!("Error sending message: {:?}", why),
            };
        };
    }
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
        println!("Task inserted");
    }
}

// pub trait Task {
//     fn run();
// }
