use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::client::Context;

use super::Task;

enum CreateChannelType {
    Team,
    Public,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChannel {
    guild_id: u64,
    category_id: u64,
    channel_name: String,
}

#[async_trait]
impl Task for CreateChannel {
    async fn handle(&self, _ctx: Arc<Context>) {}
}
