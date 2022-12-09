use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::client::Context;

use super::Task;
use crate::db_wrapper::DBWrapper;

enum CreateChannelType {
    Team,
    Public,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChannel {
    guild_id: u64,
    category_id: u64,
    channel_name: String,
}

#[async_trait]
impl Task for CreateChannel {
    async fn handle(&self, ctx: Arc<Context>, _db: DBWrapper) {
        // Create the channel
        let _channel = ctx
            .cache
            .guild(self.guild_id)
            .unwrap()
            .create_channel(&ctx.http, |c| {
                c.name(&self.channel_name);
                c.category(self.category_id);
                c
            })
            .await
            .unwrap();
    }
}
