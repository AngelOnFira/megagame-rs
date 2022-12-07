use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{builder::CreateChannel, client::Context, model::channel::ChannelType};

use super::Task;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum CreateCategoryKind {
    Team,
    Public,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategory {
    guild_id: u64,
    category_name: String,
    kind: CreateCategoryKind,
}

#[async_trait]
impl Task for CreateCategory {
    async fn handle(&self, ctx: Arc<Context>) {
        // Create the category
        let category = ctx
            .cache
            .guild(self.guild_id)
            .unwrap()
            .create_channel(&ctx.http, |c| {
                c.name(&self.category_name);
                c.kind(ChannelType::Category);
                c
            })
            .await
            .unwrap();
    }
}
