use std::{num::NonZeroU64, sync::Arc};

use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use serenity::{builder::CreateMessage, client::Context, model::prelude::ChannelId};
use tracing::log;

use super::{get_guild, DiscordId, Task, TaskTest};
use crate::db_wrapper::{DBWrapper, TaskResult, TaskReturnData};

// pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageHandler {
    pub guild_id: DiscordId,
    pub task: MessageTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageTasks {
    SendChannelMessage(SendChannelMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendChannelMessage {
    pub channel_id: DiscordId,
    pub message: String,
}

#[async_trait]
impl Task for MessageHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) -> TaskResult {
        match &self.task {
            MessageTasks::SendChannelMessage(send_channel_message) => {
                self.handle_send_channel_message(send_channel_message.clone(), ctx, db)
                    .await
            }
        }
    }
}

impl MessageHandler {
    async fn handle_send_channel_message(
        &self,
        send_channel_message: SendChannelMessage,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> TaskResult {
        let (_discord_guild, _database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        let channel_id = ChannelId(NonZeroU64::new(*send_channel_message.channel_id).unwrap());

        let message = channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().content(send_channel_message.message),
            )
            .await
            .unwrap();

        TaskResult::Completed(TaskReturnData::MessageId(DiscordId(message.id.into())))
    }
}

#[async_trait]
impl TaskTest for MessageHandler {
    async fn run_tests(_ctx: Arc<Context>, _db: DBWrapper) {
        log::info!("Testing categories");
        // assert_not_error(test_create_channel(ctx, db).await);
    }
}

// #[derive(Debug)]
// pub enum CategoryCreateError {
//     CategoryAlreadyExists,
//     CategoryNotCreated,
//     CategoryNotInDatabase,
// }
