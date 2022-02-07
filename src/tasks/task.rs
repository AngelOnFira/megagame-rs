use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::{client::Context, http::CacheHttp, model::id::UserId};

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

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub task: TaskType,
}

#[derive(Serialize, Deserialize)]
pub struct MessageUser {
    pub player_id: u64,
    pub message: String,
}

impl Task {
    pub async fn message_user(&self, ctx: Arc<Context>) {
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
