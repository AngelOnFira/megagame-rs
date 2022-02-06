use std::sync::Arc;

use serenity::client::Context;

use serenity::http::CacheHttp;
use serenity::model::id::UserId;

// use self::message::Message;

// mod message;

pub enum Tasks {
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
}

struct Task {
    task: Tasks,
}

pub struct MessageUser {
    player_id: u64,
    message: String,
}

impl Task {
    async fn message_user(&self, ctx: Arc<Context>) {
        let message = if let Tasks::MessageUser(message) = &self.task {
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

        let task = Task {
            task: Tasks::MessageUser(MessageUser {
                player_id: 133358326439346176,
                message: String::from("Good day"),
            }),
        };

        match task.task {
            Tasks::MessageUser(_) => task.message_user(Arc::clone(&self.ctx)).await,
            _ => unimplemented!(),
        }
    }
}

// pub trait Task {
//     fn run();
// }
