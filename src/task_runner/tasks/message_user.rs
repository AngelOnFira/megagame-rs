use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{client::Context, http::CacheHttp, model::id::UserId};
use tracing::log;

use crate::db_wrapper::DBWrapper;

use super::Task;

/// Send a message to a user with the provided player_id.
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct MessageUser {
    pub player_id: u64,
    pub message: String,
}

#[async_trait]
impl Task for MessageUser {
    async fn handle(&self, ctx: Arc<Context>, _db: DBWrapper) {
        if let Ok(user) = UserId(self.player_id).to_user(ctx.http()).await {
            match user
                .direct_message(ctx.http(), |m| m.content(self.message.as_str()))
                .await
            {
                Ok(_) => log::info!("Message sent"),
                Err(why) => log::error!("Error sending message: {:?}", why),
            };
        };
    }
}
